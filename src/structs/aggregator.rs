use super::common::Hash;
use super::decimal::SwitchboardDecimal;
use anchor_lang::{zero_copy, AnchorDeserialize};
use solana_program::pubkey::Pubkey;

#[zero_copy]
#[derive(AnchorDeserialize, Default, Debug, PartialEq, Eq)]
pub struct AggregatorRound {
    // Maintains the number of successful responses received from nodes.
    // Nodes can submit one successful response per round.
    pub num_success: Option<u32>,
    pub num_error: Option<u32>,
    pub is_closed: Option<bool>,
    // Maintains the `solana_program::clock::Slot` that the round was opened at.
    pub round_open_slot: Option<u64>,
    // Maintains the `solana_program::clock::UnixTimestamp;` the round was opened at.
    pub round_open_timestamp: Option<i64>,
    // Maintains the current median of all successful round responses.
    pub result: Option<SwitchboardDecimal>,
    // Standard deviation of the accepted results in the round.
    pub std_deviation: Option<SwitchboardDecimal>,
    // Maintains the minimum node response this round.
    pub min_response: Option<SwitchboardDecimal>,
    // Maintains the maximum node response this round.
    pub max_response: Option<SwitchboardDecimal>,
    // pub lease_key: Option<Pubkey>,
    // Pubkeys of the oracles fulfilling this round.
    pub oracle_pubkeys_data: Option<[Pubkey; 16]>,
    // pub oracle_pubkeys_size: Option<u32>, IMPLIED BY ORACLE_REQUEST_BATCH_SIZE
    // Represents all successful node responses this round. `NaN` if empty.
    pub medians_data: Option<[SwitchboardDecimal; 16]>,
    // Current rewards/slashes oracles have received this round.
    pub current_payout: Option<[i64; 16]>,
    // Optionals do not work on zero_copy. Keep track of which responses are
    // fulfilled here.
    pub medians_fulfilled: Option<[bool; 16]>,
    // could do specific error codes
    pub errors_fulfilled: Option<[bool; 16]>,
}

impl Default for AggregatorState {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[derive(AnchorDeserialize, Debug, PartialEq, Clone)]
pub struct AggregatorState {
    pub name: Option<[u8; 32]>,
    pub metadata: Option<[u8; 128]>,
    pub author_wallet: Option<Pubkey>,
    pub queue_pubkey: Option<Pubkey>,
    // CONFIGS
    // affects update price, shouldnt be changeable
    pub oracle_request_batch_size: Option<u32>,
    pub min_oracle_results: Option<u32>,
    pub min_job_results: Option<u32>,
    // affects update price, shouldnt be changeable
    pub min_update_delay_seconds: Option<u32>,
    // timestamp to start feed updates at
    pub start_after: Option<i64>,
    pub variance_threshold: Option<SwitchboardDecimal>,
    // If no feed results after this period, trigger nodes to report
    pub force_report_period: Option<i64>,
    pub expiration: Option<i64>,
    //
    pub consecutive_failure_count: Option<u64>,
    pub next_allowed_update_time: Option<i64>,
    pub is_locked: Option<bool>,
    pub _schedule: Option<[u8; 32]>,
    pub latest_confirmed_round: Option<AggregatorRound>,
    pub current_round: Option<AggregatorRound>,
    pub job_pubkeys_data: Option<[Pubkey; 16]>,
    pub job_hashes: Option<[Hash; 16]>,
    pub job_pubkeys_size: Option<u32>,
    // Used to confirm with oracles they are answering what they think theyre answering
    pub jobs_checksum: Option<[u8; 32]>,
    //
    pub authority: Option<Pubkey>,
    pub _ebuf: Option<[u8; 224]>, // Buffer for future info
}
