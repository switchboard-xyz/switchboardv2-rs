#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SwitchboardAccountType {
    TYPE_UNINITIALIZED = 0,
    TYPE_AGGREGATOR = 1,
    TYPE_FULFILLMENT_MANAGER = 2,
    TYPE_JOB_DEFINITION = 3,
    TYPE_FULFILLMENT_MANAGER_AUTH = 4,
    TYPE_AGGREGATOR_RESULT_PARSE_OPTIMIZED = 5,
    TYPE_BUNDLE = 6,
    TYPE_BUNDLE_AUTH = 7,
    TYPE_VRF = 8,
    TYPE_VRF_PERMIT = 9,
}

impl Default for SwitchboardAccountType {
    fn default() -> Self {
        SwitchboardAccountType::TYPE_UNINITIALIZED
    }
}

impl From<i32> for SwitchboardAccountType {
    fn from(i: i32) -> Self {
        match i {
            0 => SwitchboardAccountType::TYPE_UNINITIALIZED,
            1 => SwitchboardAccountType::TYPE_AGGREGATOR,
            2 => SwitchboardAccountType::TYPE_FULFILLMENT_MANAGER,
            3 => SwitchboardAccountType::TYPE_JOB_DEFINITION,
            4 => SwitchboardAccountType::TYPE_FULFILLMENT_MANAGER_AUTH,
            5 => SwitchboardAccountType::TYPE_AGGREGATOR_RESULT_PARSE_OPTIMIZED,
            6 => SwitchboardAccountType::TYPE_BUNDLE,
            7 => SwitchboardAccountType::TYPE_BUNDLE_AUTH,
            8 => SwitchboardAccountType::TYPE_VRF,
            9 => SwitchboardAccountType::TYPE_VRF_PERMIT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for SwitchboardAccountType {
    fn from(s: &'a str) -> Self {
        match s {
            "TYPE_UNINITIALIZED" => SwitchboardAccountType::TYPE_UNINITIALIZED,
            "TYPE_AGGREGATOR" => SwitchboardAccountType::TYPE_AGGREGATOR,
            "TYPE_FULFILLMENT_MANAGER" => SwitchboardAccountType::TYPE_FULFILLMENT_MANAGER,
            "TYPE_JOB_DEFINITION" => SwitchboardAccountType::TYPE_JOB_DEFINITION,
            "TYPE_FULFILLMENT_MANAGER_AUTH" => {
                SwitchboardAccountType::TYPE_FULFILLMENT_MANAGER_AUTH
            }
            "TYPE_AGGREGATOR_RESULT_PARSE_OPTIMIZED" => {
                SwitchboardAccountType::TYPE_AGGREGATOR_RESULT_PARSE_OPTIMIZED
            }
            "TYPE_BUNDLE" => SwitchboardAccountType::TYPE_BUNDLE,
            "TYPE_BUNDLE_AUTH" => SwitchboardAccountType::TYPE_BUNDLE_AUTH,
            "TYPE_VRF" => SwitchboardAccountType::TYPE_VRF,
            "TYPE_VRF_PERMIT" => SwitchboardAccountType::TYPE_VRF_PERMIT,
            _ => Self::default(),
        }
    }
}
