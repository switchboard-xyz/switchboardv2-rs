pub mod aggregator;
pub mod decimal;
pub mod error;
pub use aggregator::AggregatorAccountData;
pub use anchor_lang::prelude::*;
pub use decimal::SwitchboardDecimal;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;

// declare_id!("3TSYZ3oXt9e42TdXFuYVaQyLP5ZX3fWtMhaQReHPau44");

pub fn get_aggregator_result(
    switchboard_feed: &AccountInfo,
) -> Result<SwitchboardDecimal, ProgramError> {
    let aggregator = AggregatorAccountData::new(switchboard_feed)?;
    aggregator.get_result()
}
