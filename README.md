# switchboard-aggregator

A Rust library to interact with Switchboard V2's hosted data feeds.

## Description

This package can be used to manage Switchboard data feed account parsing.

Specifically, this package will return the most recent confirmed round result
from a provided data feed AccountInfo.

## Usage

```rust
use switchboard_aggregator::get_aggregator_result;
use switchboard_aggregator::decimal::SwitchboardDecimal;
use std::convert::TryInto;

let aggregator_result: SwitchboardDecimal = switchboard_aggregator::get_aggregator_result(
    switchboard_feed // &AccountInfo
)?;

let decimal: f64 = (&aggregator_result).try_into().unwrap();
```
