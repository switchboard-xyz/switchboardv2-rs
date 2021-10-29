use anchor_lang::zero_copy;

#[zero_copy]
#[derive(Serialize, Deserialize, Default, Eq, PartialEq, Debug)]
pub struct SwitchboardDecimal {
    pub mantissa: i128,
    pub scale: Option<u32>,
}
