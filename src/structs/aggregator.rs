#[allow(unaligned_references)]
use super::common::Hash;
use super::decimal::SwitchboardDecimal;
use super::error::SwitchboardError;
use anchor_lang::prelude::*;
use anchor_lang::AnchorDeserialize;

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
    pub fn is_round_valid(&self, min_oracle_results: u32) -> bool {
        if self.num_success >= min_oracle_results {
            return true;
        }
        false
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

    pub fn get_result(&self) -> Result<AggregatorRound, ProgramError> {
        if self.current_round.is_round_valid(self.min_oracle_results) {
            Ok(self.current_round)
        } else if self
            .latest_confirmed_round
            .is_round_valid(self.min_oracle_results)
        {
            Ok(self.latest_confirmed_round)
        } else {
            Err(ProgramError::from(SwitchboardError::InvalidAggregatorRound))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_aggregator(
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
    fn create_round(num_success: u32, num_error: u32, v: f64) -> AggregatorRound {
        let mut result = AggregatorRound::default();
        result.num_success = num_success;
        result.num_error = num_error;
        result.result = SwitchboardDecimal::from_f64(v);
        return result;
    }

    #[test]
    fn test_reject_current_on_sucess_count() {
        let current_round = create_round(2, 5, 97.5);
        let last_round = create_round(30, 0, 100.0);

        let aggregator = create_aggregator(current_round.clone(), last_round.clone());
        assert_eq!(aggregator.get_result().unwrap(), last_round.clone());
    }

    #[test]
    fn test_accept_current_on_sucess_count() {
        let current_round = create_round(20, 5, 97.5);
        let last_round = create_round(30, 0, 100.0);

        let aggregator = create_aggregator(current_round.clone(), last_round.clone());
        assert_eq!(aggregator.get_result().unwrap(), current_round.clone());
    }

    #[test]
    fn test_no_valid_aggregator_result() {
        let current_round = create_round(1, 5, 97.5);
        let last_round = create_round(1, 5, 100.0);

        let aggregator = create_aggregator(current_round.clone(), last_round.clone());
        assert_eq!(
            aggregator.get_result(),
            Err(ProgramError::from(SwitchboardError::InvalidAggregatorRound))
        );
    }
}
