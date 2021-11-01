# switchboard-aggregator

A Rust library to interact with Switchboard V2's hosted data feeds.

## Description

This package can be used to manage Switchboard data feed account parsing.

Specifically, this package will return the most recent confirmed round result
from a provided data feed AccountInfo.

## Usage

```rust
use switchboard_aggregator;
use switchboard_aggregator::structs::AggregatorRound;

let aggregator_result: AggregatorRound = switchboard_aggregator::get_aggregator_result(
    switchboard_feed_account // &AccountInfo
)?;

// pub struct AggregatorRound {
//     pub num_success: u32,
//     pub num_error: u32,
//     pub is_closed: bool,
//     pub round_open_slot: u64,
//     pub round_open_timestamp: i64,
//     pub result: SwitchboardDecimal,
//     pub std_deviation: SwitchboardDecimal,
//     pub min_response: SwitchboardDecimal,
//     pub max_response: SwitchboardDecimal,
//     pub oracle_pubkeys_data: [Pubkey; 16],
//     pub medians_data: [SwitchboardDecimal; 16],
//     pub current_payout: [i64; 16],
//     pub medians_fulfilled: [bool; 16],
//     pub errors_fulfilled: [bool; 16],
// }

```
