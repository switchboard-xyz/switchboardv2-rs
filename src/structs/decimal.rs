use anchor_lang::{zero_copy, AnchorDeserialize};
use std::convert::From;

#[zero_copy]
#[derive(AnchorDeserialize, Default, Eq, PartialEq, Debug)]
pub struct SwitchboardDecimal {
    pub mantissa: i128,
    pub scale: Option<u32>,
}
