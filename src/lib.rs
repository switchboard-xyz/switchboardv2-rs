pub mod structs;
pub use anchor_lang::prelude::*;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use std::convert::TryInto;
use std::str::FromStr;
pub use structs::{AggregatorAccountData, AggregatorRound, SwitchboardDecimal};

declare_id!("3TSYZ3oXt9e42TdXFuYVaQyLP5ZX3fWtMhaQReHPau44");

const SWITCHBOARD_DEVNET_PID: &str = "5n43jDh58UzjqGE2sFuZPrkgx52BT6PWgcdL1CvBU9Ww";

pub fn get_aggregator_result_devnet(switchboard_feed: &AccountInfo) -> Result<f64, ProgramError> {
    let pid = Pubkey::from_str(&SWITCHBOARD_DEVNET_PID).unwrap();
    get_aggregator_result(switchboard_feed, &pid)
}

const SWITCHBOARD_MAINNET_PID: &str = "5n43jDh58UzjqGE2sFuZPrkgx52BT6PWgcdL1CvBU9Ww";

pub fn get_aggregator_result_mainnet(switchboard_feed: &AccountInfo) -> Result<f64, ProgramError> {
    let pid = Pubkey::from_str(&SWITCHBOARD_MAINNET_PID).unwrap();
    get_aggregator_result(switchboard_feed, &pid)
}

fn get_aggregator_result(
    switchboard_feed: &AccountInfo,
    program_id: &Pubkey,
) -> Result<f64, ProgramError> {
    let aggregator_account_loader =
        Loader::<AggregatorAccountData>::try_from(program_id, switchboard_feed)?;
    let aggregator = aggregator_account_loader.load()?;
    let round = aggregator.get_result()?;
    let result = &round.result;
    let final_result: f64 = result.try_into()?;
    Ok(final_result)
}
