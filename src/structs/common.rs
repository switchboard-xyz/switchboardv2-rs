use anchor_lang::zero_copy;

#[zero_copy]
#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct Hash {
    pub data: Option<[u8; 32]>,
}
