use anchor_lang::prelude::*;

#[zero_copy]
#[derive(AnchorDeserialize, Default, Debug, PartialEq, Eq)]
pub struct Hash {
    pub data: [u8; 32],
}
