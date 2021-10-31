use anchor_lang::{zero_copy, AnchorDeserialize};

#[zero_copy]
#[derive(AnchorDeserialize, Default, Debug, PartialEq, Eq)]
pub struct Hash {
    pub data: [u8; 32],
}
