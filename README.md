# switchboard-aggregator

A Rust library to interact with Switchboard V2's hosted data feeds.

## Description

This package can be used to manage Switchboard data feed account parsing.

Specifically, this package will return the most recent confirmed round result
from a provided data feed AccountInfo.

## Usage

```rust
use switchboard_aggregator::AggregatorAccountData;
use std::convert::TryInto;

let feed_result = AggregatorAccountData::new(feed_account_info)?.get_result()?;

let decimal: f64 = feed_result.try_into()?;
```
