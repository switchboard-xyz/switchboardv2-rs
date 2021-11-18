use anchor_lang::prelude::*;

#[anchor_lang::error]
pub enum SwitchboardError {
    #[msg("Aggregator is not currently populated with a valid round.")]
    InvalidAggregatorRound,
    #[msg("Failed to convert string to decimal format.")]
    InvalidStrDecimalConversion,
    #[msg("Decimal conversion method failed.")]
    DecimalConversionError,
    #[msg("An integer overflow occurred.")]
    IntegerOverflowError,
    #[msg("Account discriminator did not match.")]
    AccountDiscriminatorMismatch,
}
