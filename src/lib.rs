pub mod structs;
use anchor_lang::{zero_copy, AnchorDeserialize, AnchorSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
pub use structs::{AggregatorRound, AggregatorState, SwitchboardAccountType, SwitchboardDecimal};

/// Returns whether the current open round is considered valid for usage.
pub fn is_current_round_valid(aggregator: &AggregatorState) -> Result<bool, ProgramError> {
    let maybe_round = aggregator.current_round.clone();
    if maybe_round.is_none() {
        return Ok(false);
    }
    let min_confirmations = aggregator.min_oracle_results.unwrap();
    let round = maybe_round.unwrap();
    let num_success = round.num_success.unwrap();
    if num_success < min_confirmations {
        return Ok(false);
    }
    Ok(true)
}

/// Given a Switchboard data feed account, this method will parse the account state.
///
/// Returns a ProgramError if the AccountInfo is unable to be borrowed or the
/// account is not initialized as an aggregator.
pub fn get_aggregator(switchboard_feed: &AccountInfo) -> Result<AggregatorState, ProgramError> {
    let state_buffer = switchboard_feed.try_borrow_data()?;
    if state_buffer.len() == 0 || state_buffer[0] != SwitchboardAccountType::TYPE_AGGREGATOR as u8 {
        return Err(ProgramError::InvalidAccountData);
    }
    let aggregator_state: AggregatorState =
        AnchorDeserialize::try_from_slice(&state_buffer).unwrap();
    Ok(aggregator_state)
}

/// Returns the most recent resolution round that is considered valid for the aggregator.
pub fn get_aggregator_result(
    aggregator: &AggregatorState,
) -> Result<AggregatorRound, ProgramError> {
    let mut maybe_round = aggregator.current_round.clone();
    if !is_current_round_valid(&aggregator)? {
        maybe_round = aggregator.latest_confirmed_round.clone();
    }
    maybe_round.ok_or(ProgramError::InvalidAccountData)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn create_aggregator(
        current_round: AggregatorRound,
        last_round: AggregatorRound,
    ) -> AggregatorState {
        let mut aggregator = AggregatorState::default();
        aggregator.min_update_delay_seconds = Some(10);
        aggregator.latest_confirmed_round = Some(last_round);
        aggregator.current_round = Some(current_round);
        aggregator.min_job_results = Some(10);
        aggregator.min_oracle_results = Some(10);
        return aggregator;
    }

    #[test]
    fn test_reject_current_on_sucess_count() {
        let mut current_round = AggregatorRound::default();
        current_round.num_success = Some(2);
        current_round.num_error = Some(5);
        current_round.result = Some(SwitchboardDecimal {
            mantissa: 975,
            scale: Some(1),
        });
        current_round.round_open_slot = Some(1);
        current_round.round_open_timestamp = Some(1);
        current_round.min_response = Some(SwitchboardDecimal {
            mantissa: 961,
            scale: Some(1),
        });
        current_round.max_response = Some(SwitchboardDecimal {
            mantissa: 100,
            scale: None,
        });

        let mut last_round = AggregatorRound::default();
        last_round.num_success = Some(30);
        last_round.num_error = Some(0);
        last_round.result = Some(SwitchboardDecimal {
            mantissa: 100,
            scale: None,
        });
        last_round.round_open_slot = Some(1);
        last_round.round_open_timestamp = Some(1);
        last_round.min_response = Some(SwitchboardDecimal {
            mantissa: 100,
            scale: None,
        });
        last_round.max_response = Some(SwitchboardDecimal {
            mantissa: 100,
            scale: None,
        });

        let aggregator = create_aggregator(current_round.clone(), last_round.clone());
        assert_eq!(
            get_aggregator_result(&aggregator).unwrap(),
            last_round.clone()
        );
    }

    #[test]
    fn test_accept_current_on_sucess_count() {
        let mut current_round = AggregatorRound::default();
        current_round.num_success = Some(20);
        current_round.num_error = Some(5);
        current_round.result = Some(SwitchboardDecimal {
            mantissa: 970,
            scale: Some(1),
        });
        current_round.round_open_slot = Some(1);
        current_round.round_open_timestamp = Some(1);
        current_round.min_response = Some(SwitchboardDecimal {
            mantissa: 961,
            scale: Some(1),
        });
        current_round.max_response = Some(SwitchboardDecimal {
            mantissa: 100,
            scale: None,
        });
        let mut last_round = AggregatorRound::default();
        last_round.num_success = Some(30);
        last_round.num_error = Some(0);
        last_round.result = Some(SwitchboardDecimal {
            mantissa: 100,
            scale: None,
        });
        last_round.round_open_slot = Some(1);
        last_round.round_open_timestamp = Some(1);
        last_round.min_response = Some(SwitchboardDecimal {
            mantissa: 100,
            scale: None,
        });
        last_round.max_response = Some(SwitchboardDecimal {
            mantissa: 100,
            scale: None,
        });

        let aggregator = create_aggregator(current_round.clone(), last_round.clone());
        assert_eq!(
            get_aggregator_result(&aggregator).unwrap(),
            current_round.clone()
        );
    }
}
