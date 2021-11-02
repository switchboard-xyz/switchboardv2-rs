pub mod structs;
pub use anchor_lang::prelude::*;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
pub use structs::{AggregatorAccountData, SwitchboardDecimal, SwitchboardError};

declare_id!("3TSYZ3oXt9e42TdXFuYVaQyLP5ZX3fWtMhaQReHPau44");

const SWITCHBOARD_DEVNET_PID: &str = "5n43jDh58UzjqGE2sFuZPrkgx52BT6PWgcdL1CvBU9Ww";

pub fn get_aggregator_result_devnet(
    switchboard_feed: &AccountInfo,
) -> Result<SwitchboardDecimal, ProgramError> {
    let pid = Pubkey::from_str(&SWITCHBOARD_DEVNET_PID).unwrap();
    get_aggregator_result(switchboard_feed, &pid)
}

const SWITCHBOARD_MAINNET_PID: &str = "5n43jDh58UzjqGE2sFuZPrkgx52BT6PWgcdL1CvBU9Ww";

pub fn get_aggregator_result_mainnet(
    switchboard_feed: &AccountInfo,
) -> Result<SwitchboardDecimal, ProgramError> {
    let pid = Pubkey::from_str(&SWITCHBOARD_MAINNET_PID).unwrap();
    get_aggregator_result(switchboard_feed, &pid)
}

fn get_aggregator_result(
    switchboard_feed: &AccountInfo,
    program_id: &Pubkey,
) -> Result<SwitchboardDecimal, ProgramError> {
    let aggregator_account_loader =
        Loader::<AggregatorAccountData>::try_from(program_id, switchboard_feed)?;
    let aggregator = aggregator_account_loader.load()?;
    if aggregator.min_oracle_results > aggregator.latest_confirmed_round.num_success {
        return Err(ProgramError::from(SwitchboardError::InvalidAggregatorRound));
    }
    Ok(aggregator.latest_confirmed_round.result)
}
