use anchor_lang::{zero_copy, AnchorDeserialize, AnchorSerialize};

#[zero_copy]
#[derive(AnchorDeserialize, AnchorSerialize, Default, Eq, PartialEq, Debug)]
pub struct SwitchboardDecimal {
    pub mantissa: i128,
    pub scale: Option<u32>,
}
