pub mod structs;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
pub use structs::{AggregatorAccountData, AggregatorRound, SwitchboardDecimal};

anchor_lang::declare_id!("3TSYZ3oXt9e42TdXFuYVaQyLP5ZX3fWtMhaQReHPau5f");

/// Given a Switchboard data feed account, this method will parse the account state.
///
/// Returns the most recent resolution round that is considered valid for the aggregator.
pub fn get_aggregator_result(
    switchboard_feed: &AccountInfo,
) -> Result<AggregatorRound, ProgramError> {
    let aggregator = AggregatorAccountData::new(switchboard_feed)?;
    aggregator.get_result()
}


