pub mod structs;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
pub use structs::{AggregatorAccountData, AggregatorRound, SwitchboardDecimal};

anchor_lang::declare_id!("GFerzWEGnbUn4ZvjVqdKhasNSFU16BkbYB1eUqxHMtDA");

/// Given a Switchboard data feed account, this method will parse the account state.
///
/// Returns the most recent resolution round that is considered valid for the aggregator.
pub fn get_aggregator_result(
    switchboard_feed: &AccountInfo,
) -> Result<u64, ProgramError> {
    msg!("{}", std::mem::size_of::<AggregatorAccountData>());
    msg!("a");
    // let aggregator = AggregatorAccountData::new(switchboard_feed)?;
    // msg!("a");
    // aggregator.get_result()
    0
}
