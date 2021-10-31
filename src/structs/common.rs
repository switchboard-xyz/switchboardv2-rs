use anchor_lang::{zero_copy, AnchorDeserialize, AnchorSerialize};

#[zero_copy]
#[derive(AnchorDeserialize, AnchorSerialize, Default, Debug, PartialEq, Eq)]
pub struct Hash {
    pub data: Option<[u8; 32]>,
}
