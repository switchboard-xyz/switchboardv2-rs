#[anchor_lang::error]
pub enum SwitchboardError {
    #[msg("Aggregator is not currently populated with a valid round.")]
    InvalidAggregatorRound,
    #[msg("Aggregator cannot fit any more jobs.")]
    TooManyAggregatorJobs,
    #[msg("Aggregator's current round is closed. No results are being accepted.")]
    AggregatorCurrentRoundClosed,
    #[msg("Aggregator received an invalid save result instruction.")]
    AggregatorInvalidSaveResult,
    #[msg("Failed to convert string to decimal format.")]
    InvalidStrDecimalConversion,
    #[msg("Loader account is missing a required signature.")]
    AggregatorAccountNotFound,
    #[msg("Permission accont missing from provided account list.")]
    PermissionAccountNotFound,
    #[msg("Failed to derive a lease account.")]
    LeaseAccountDeriveFailure,
    #[msg("Failed to derive a permission account.")]
    PermissionAccountDeriveFailure,
    #[msg("Escrow accont missing from provided account list.")]
    EscrowAccountNotFound,
    #[msg("Lease accont missing from provided account list.")]
    LeaseAccountNotFound,
    #[msg("Decimal conversion method failed.")]
    DecimalConversionError,
    #[msg("Aggregator invalid batch size.")]
    AggregatorInvalidBatchSizeError,
    #[msg("Oracle provided an incorrect aggregator job checksum.")]
    AggregatorJobChecksumMismatch,
    #[msg("An integer overflow occurred.")]
    IntegerOverflowError,
}
