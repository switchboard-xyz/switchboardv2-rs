use anchor_lang::{zero_copy, AnchorDeserialize};
use rust_decimal::prelude::*;
use std::convert::From;

#[zero_copy]
#[derive(AnchorDeserialize, Default, Eq, PartialEq, Debug)]
pub struct SwitchboardDecimal {
    pub mantissa: i128,
    pub scale: Option<u32>,
}

impl From<i64> for SwitchboardDecimal {
    fn from(item: i64) -> Self {
        SwitchboardDecimal {
            mantissa: item as i128,
            scale: None,
        }
    }
}

impl From<i32> for SwitchboardDecimal {
    fn from(item: i32) -> Self {
        SwitchboardDecimal {
            mantissa: item as i128,
            scale: None,
        }
    }
}

impl From<SwitchboardDecimal> for Decimal {
    fn from(item: SwitchboardDecimal) -> Self {
        Decimal::from_i128_with_scale(item.mantissa, item.scale.unwrap_or_default())
    }
}
