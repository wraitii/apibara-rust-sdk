use std::error::Error;

use apibara_client_protos::pb::starknet::v1alpha2::FieldElement;
use apibara_client_protos::pb::stream::v1alpha2::stream_data_response::Message;
use apibara_client_protos::pb::{stream::v1alpha2::{stream_client::StreamClient, Cursor, StreamDataRequest, StreamDataResponse}, starknet::v1alpha2::{EventFilter, HeaderFilter}};
use client::ApibaraClient;
use futures::StreamExt;
use sqlite::Value;
use tokio::sync::mpsc;
use tonic::{transport::{Channel, ClientTlsConfig, Certificate}, IntoStreamingRequest, Response};

use log::{info, debug, warn};
use hex_literal::hex;

use num_bigint::{BigUint, BigInt, ToBigInt, ToBigUint};
use num_traits::{ToPrimitive};
mod client;

#[tokio::main]
async fn main() {
    env_logger::init();

    let stream = client::ApibaraClient::new(client::Chain::AlphaMainnet.into()).await;
    match stream {
        Ok(s) => {
            println!("Connected");
            do_stuff(s).await;
        },
        Err(e) => println!("Error: {:?}", e),
    }
}

async fn do_stuff(mut stream: ApibaraClient) -> Result<(), Box<dyn Error>> {
    let contract: [u8; 32] = hex!("01435498bf393da86b4733b9264a86b58a42b31f8d8b8ba309593e5c17847672");
    /*
    const stream_id: u64 = 243;
    let request = StreamDataRequest {
        stream_id: Some(stream_id),
        batch_size: Some(1),
        starting_cursor: Some(Cursor { order_key: 1, unique_key: vec![] }),
        finality: None,
        filter: Some(Filter { header: Some(HeaderFilter { weak: true }), transactions: vec![], state_update: None,
            events: vec![EventFilter {
                from_address: Some(FieldElement::from_bytes(&contract)),
                keys: vec![FieldElement::from_bytes(&hex!("0099cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9"))],
                data: vec![]
            }],
            messages: vec![]
        })
    };
    */

    let connection = sqlite::open(":memory:").unwrap();

    let query = "
        CREATE TABLE set_transfers (address_from TEXT, address_to TEXT, token TEXT, _chain_valid_from INTEGER, _chain_valid_to INTEGER);
    ";
    connection.execute(query).unwrap();

    let query = "INSERT INTO set_transfers VALUES (?, ?, ?, ?, ?)";

    let mut data_stream = stream.request_data(client::Filter { contract /*header: Some(HeaderFilter { weak: true }), transactions: vec![], state_update: None,
        events: vec![EventFilter {
            from_address: Some(FieldElement::from_bytes(&contract)),
            keys: vec![FieldElement::from_bytes(&hex!("0099cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9"))],
            data: vec![]
        }],
        messages: vec![]
    */}.into()).await?;
    
    futures::pin_mut!(data_stream);

    while let Some(mess) = data_stream.next().await {
        match mess {
            Ok(Some(mess)) => {
                debug!("Received message");
                let data = &mess.data;
                // TODO: pending data
                //let end_cursor = &data.end_cursor;
                //let cursor = &data.cursor;
                for block in data {
                    match &block.header {
                        Some(header) => {
                            info!("Received block {}", header.block_number);
                        },
                        None => {
                            warn!("Received block without header");
                        }
                    }
                    for event in &block.events {
                        let tx_hash = &event.transaction.as_ref().unwrap().meta.as_ref().unwrap().hash.as_ref().unwrap().to_biguint();
                        match &event.event {
                            Some(ev_data) => {
                                let address = ev_data.from_address.as_ref().unwrap().to_hex_string();
                                let data: Vec<BigUint> = ev_data.data.iter().map(|item| item.to_biguint()).collect();
                                let from = data[0].to_str_radix(16);
                                let to = data[1].to_str_radix(16);
                                let token_id = (&data[2] + &data[3] * 2.to_biguint().unwrap().pow(128)).to_str_radix(16);
                                info!("Received event from 0x{} to 0x{} token 0x{} in TX 0x{}", from, to, token_id, tx_hash.to_str_radix(16));
                                let mut statement = connection.prepare(query).unwrap();
                                statement.bind::<&[(_, Value)]>(&[
                                    (1, from.into()),
                                    (2, to.into()),
                                    (3, token_id.into()),
                                    (4, block.header.as_ref().unwrap().block_number.to_i64().unwrap_or(0).into()),
                                    (5, sqlite::Value::Null)
                                ])?;
                                statement.next();
                            },
                            None => {
                                warn!("Received event without key");
                            }
                        }
                    }
                }
            },
            Ok(None) => {
                continue;
            }
            Err(e) => {
                warn!("Error: {:?}", e);
            }
        }
    }
        
        /*
        logger.debug("received message")
        self._retry_count = 0
        if message.data is not None:
            assert (
                len(message.data.data) <= 1
            ), "indexer runner requires batch_size == 1"

            # invalidate any pending data, if any
            if pending_received and previous_end_cursor is not None:
                self._indexer_storage.invalidate(previous_end_cursor)

            is_pending = message.data.finality == DataFinality.DATA_STATUS_PENDING
            pending_received = is_pending

            end_cursor = message.data.end_cursor
            cursor = message.data.cursor
            for batch in message.data.data:
                with self._indexer_storage.create_storage_for_data(
                    message.data.end_cursor
                ) as storage:
                    decoded_data = indexer.decode_data(batch)
                    info = Info(
                        context=ctx,
                        storage=storage,
                        cursor=cursor,
                        end_cursor=cursor,
                    )
                    if is_pending:
                        await indexer.handle_pending_data(info, decoded_data)
                    else:
                        await indexer.handle_data(info, decoded_data)
                    # TODO: check if user updated filter

            if not is_pending:
                previous_end_cursor = message.data.end_cursor

        elif message.invalidate is not None:
            with self._indexer_storage.create_storage_for_invalidate(
                message.invalidate.cursor
            ) as storage:
                cursor = message.invalidate.cursor
                info = Info(
                    context=ctx, storage=storage, cursor=cursor, end_cursor=cursor
                )
                await indexer.handle_invalidate(info, cursor)
            previous_end_cursor = message.invalidate.cursor */
    println!("End of stream");
    Ok(())
}