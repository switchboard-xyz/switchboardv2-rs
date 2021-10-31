use super::common::Hash;
use super::decimal::SwitchboardDecimal;
use super::error::SwitchboardError;
use anchor_lang::AnchorDeserialize;

use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[zero_copy]
#[derive(AnchorDeserialize, Default, Debug, PartialEq, Eq)]
pub struct AggregatorRound {
    // Maintains the number of successful responses received from nodes.
    // Nodes can submit one successful response per round.
    pub num_success: u32,
    pub num_error: u32,
    pub is_closed: bool,
    // Maintains the `solana_program::clock::Slot` that the round was opened at.
    pub round_open_slot: u64,
    // Maintains the `solana_program::clock::UnixTimestamp;` the round was opened at.
    pub round_open_timestamp: i64,
    // Maintains the current median of all successful round responses.
    pub result: SwitchboardDecimal,
    // Standard deviation of the accepted results in the round.
    pub std_deviation: SwitchboardDecimal,
    // Maintains the minimum node response this round.
    pub min_response: SwitchboardDecimal,
    // Maintains the maximum node response this round.
    pub max_response: SwitchboardDecimal,
    // pub lease_key: Option<Pubkey>,
    // Pubkeys of the oracles fulfilling this round.
    pub oracle_pubkeys_data: [Pubkey; 16],
    // pub oracle_pubkeys_size: Option<u32>, IMPLIED BY ORACLE_REQUEST_BATCH_SIZE
    // Represents all successful node responses this round. `NaN` if empty.
    pub medians_data: [SwitchboardDecimal; 16],
    // Current rewards/slashes oracles have received this round.
    pub current_payout: [i64; 16],
    // Optionals do not work on zero_copy. Keep track of which responses are
    // fulfilled here.
    pub medians_fulfilled: [bool; 16],
    // could do specific error codes
    pub errors_fulfilled: [bool; 16],
}
impl AggregatorRound {
    pub fn is_round_valid(&self, min_oracle_results: u32) -> Result<bool, ProgramError> {
        if self.num_success < min_oracle_results {
            return Ok(false);
        }
        Ok(true)
    }
}

impl Default for AggregatorAccountData {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[account(zero_copy)]
#[derive(AnchorDeserialize, Debug, PartialEq)]
pub struct AggregatorAccountData {
    pub name: [u8; 32],
    pub metadata: [u8; 128],
    pub author_wallet: Pubkey,
    pub queue_pubkey: Pubkey,
    // CONFIGS
    // affects update price, shouldnt be changeable
    pub oracle_request_batch_size: u32,
    pub min_oracle_results: u32,
    pub min_job_results: u32,
    // affects update price, shouldnt be changeable
    pub min_update_delay_seconds: u32,
    // timestamp to start feed updates at
    pub start_after: i64,
    pub variance_threshold: SwitchboardDecimal,
    // If no feed results after this period, trigger nodes to report
    pub force_report_period: i64,
    pub expiration: i64,
    //
    pub consecutive_failure_count: u64,
    pub next_allowed_update_time: i64,
    pub is_locked: bool,
    pub _schedule: [u8; 32],
    pub latest_confirmed_round: AggregatorRound,
    pub current_round: AggregatorRound,
    pub job_pubkeys_data: [Pubkey; 16],
    pub job_hashes: [Hash; 16],
    pub job_pubkeys_size: u32,
    // Used to confirm with oracles they are answering what they think theyre answering
    pub jobs_checksum: [u8; 32],
    //
    pub authority: Pubkey,
    pub _ebuf: [u8; 224], // Buffer for future info
}

impl AggregatorAccountData {
    pub fn new(switchboard_feed: &AccountInfo) -> Result<AggregatorAccountData, ProgramError> {
        let aggregator_account_loader =
            AccountLoader::<AggregatorAccountData>::try_from(switchboard_feed)?;
        let aggregator: AggregatorAccountData = *aggregator_account_loader.load()?;
        Ok(aggregator)
    }

    pub fn is_current_round_valid(&self) -> Result<bool, ProgramError> {
        let round = self.current_round.clone();
        round.is_round_valid(self.min_oracle_results)
    }
    pub fn is_latest_confirmed_round_valid(&self) -> Result<bool, ProgramError> {
        let round = self.latest_confirmed_round.clone();
        round.is_round_valid(self.min_oracle_results)
    }
    pub fn get_result(&self) -> Result<AggregatorRound, ProgramError> {
        if self.is_current_round_valid().unwrap() {
            Ok(self.current_round)
        } else if self.is_latest_confirmed_round_valid().unwrap() {
            Ok(self.latest_confirmed_round)
        } else {
            Err(ProgramError::from(SwitchboardError::InvalidAggregatorRound))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn create_aggregator(
        current_round: AggregatorRound,
        last_round: AggregatorRound,
    ) -> AggregatorAccountData {
        let mut aggregator = AggregatorAccountData::default();
        aggregator.min_update_delay_seconds = 10;
        aggregator.latest_confirmed_round = last_round;
        aggregator.current_round = current_round;
        aggregator.min_job_results = 10;
        aggregator.min_oracle_results = 10;
        return aggregator;
    }

    #[test]
    fn test_reject_current_on_sucess_count() {
        let mut current_round = AggregatorRound::default();
        current_round.num_success = 2;
        current_round.num_error = 5;
        current_round.result = SwitchboardDecimal {
            mantissa: 975,
            scale: 1,
        };
        current_round.round_open_slot = 1;
        current_round.round_open_timestamp = 1;
        current_round.min_response = SwitchboardDecimal {
            mantissa: 961,
            scale: 1,
        };
        current_round.max_response = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };

        let mut last_round = AggregatorRound::default();
        last_round.num_success = 30;
        last_round.num_error = 0;
        last_round.result = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };
        last_round.round_open_slot = 1;
        last_round.round_open_timestamp = 1;
        last_round.min_response = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };
        last_round.max_response = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };

        let aggregator = create_aggregator(current_round.clone(), last_round.clone());
        assert_eq!(aggregator.get_result().unwrap(), last_round.clone());
    }

    #[test]
    fn test_accept_current_on_sucess_count() {
        let mut current_round = AggregatorRound::default();
        current_round.num_success = 20;
        current_round.num_error = 5;
        current_round.result = SwitchboardDecimal {
            mantissa: 975,
            scale: 1,
        };
        current_round.round_open_slot = 1;
        current_round.round_open_timestamp = 1;
        current_round.min_response = SwitchboardDecimal {
            mantissa: 961,
            scale: 1,
        };
        current_round.max_response = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };

        let mut last_round = AggregatorRound::default();
        last_round.num_success = 30;
        last_round.num_error = 0;
        last_round.result = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };
        last_round.round_open_slot = 1;
        last_round.round_open_timestamp = 1;
        last_round.min_response = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };
        last_round.max_response = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };

        let aggregator = create_aggregator(current_round.clone(), last_round.clone());
        assert_eq!(aggregator.get_result().unwrap(), current_round.clone());
    }

    #[test]
    fn test_no_valid_aggregator_result() {
        let mut current_round = AggregatorRound::default();
        current_round.num_success = 1;
        current_round.num_error = 5;
        current_round.result = SwitchboardDecimal {
            mantissa: 975,
            scale: 1,
        };
        current_round.round_open_slot = 1;
        current_round.round_open_timestamp = 1;
        current_round.min_response = SwitchboardDecimal {
            mantissa: 961,
            scale: 1,
        };
        current_round.max_response = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };

        let mut last_round = AggregatorRound::default();
        last_round.num_success = 1;
        last_round.num_error = 0;
        last_round.result = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };
        last_round.round_open_slot = 1;
        last_round.round_open_timestamp = 1;
        last_round.min_response = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };
        last_round.max_response = SwitchboardDecimal {
            mantissa: 100,
            scale: 0,
        };

        let aggregator = create_aggregator(current_round.clone(), last_round.clone());
        assert_eq!(
            aggregator.get_result(),
            Err(ProgramError::from(SwitchboardError::InvalidAggregatorRound))
        );
    }
}
