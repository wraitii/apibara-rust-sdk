pub mod pb {
    pub mod starknet {
        pub mod v1alpha2 {
            tonic::include_proto!("apibara.starknet.v1alpha2");

            impl FieldElement {
                pub fn from_u64(value: u64) -> FieldElement {
                    FieldElement {
                        lo_lo: 0,
                        lo_hi: 0,
                        hi_lo: 0,
                        hi_hi: value,
                    }
                }

                pub fn from_bytes(bytes: &[u8; 32]) -> Self {
                    let lo_lo = u64::from_be_bytes([
                        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
                        bytes[7],
                    ]);
                    let lo_hi = u64::from_be_bytes([
                        bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                        bytes[15],
                    ]);
                    let hi_lo = u64::from_be_bytes([
                        bytes[16], bytes[17], bytes[18], bytes[19], bytes[20], bytes[21],
                        bytes[22], bytes[23],
                    ]);
                    let hi_hi = u64::from_be_bytes([
                        bytes[24], bytes[25], bytes[26], bytes[27], bytes[28], bytes[29],
                        bytes[30], bytes[31],
                    ]);

                    FieldElement {
                        lo_lo,
                        lo_hi,
                        hi_lo,
                        hi_hi,
                    }
                }

                pub fn to_bytes(&self) -> [u8; 32] {
                    let lo_lo = self.lo_lo.to_be_bytes();
                    let lo_hi = self.lo_hi.to_be_bytes();
                    let hi_lo = self.hi_lo.to_be_bytes();
                    let hi_hi = self.hi_hi.to_be_bytes();
                    [
                        lo_lo[0], lo_lo[1], lo_lo[2], lo_lo[3], lo_lo[4], lo_lo[5], lo_lo[6],
                        lo_lo[7], lo_hi[0], lo_hi[1], lo_hi[2], lo_hi[3], lo_hi[4], lo_hi[5],
                        lo_hi[6], lo_hi[7], hi_lo[0], hi_lo[1], hi_lo[2], hi_lo[3], hi_lo[4],
                        hi_lo[5], hi_lo[6], hi_lo[7], hi_hi[0], hi_hi[1], hi_hi[2], hi_hi[3],
                        hi_hi[4], hi_hi[5], hi_hi[6], hi_hi[7],
                    ]
                }

                pub fn to_biguint(&self) -> num_bigint::BigUint {
                    num_bigint::BigUint::from_bytes_be(&self.to_bytes())
                }

                // No 0x prefix
                pub fn to_hex_string(&self) -> String {
                    hex::encode(self.to_bytes())
                }
            }
         }
    }
    pub mod stream {
        pub mod v1alpha2 {
            tonic::include_proto!("apibara.node.v1alpha2");
        }
    }
}

