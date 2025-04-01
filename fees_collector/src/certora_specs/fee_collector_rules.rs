use cvlr::clog;
use soroban_sdk::Env;

use cvlr::asserts::{cvlr_assert, cvlr_assume};
use cvlr_soroban_derive::rule;
use cvlr_soroban::nondet_address;

pub use crate::contract::FeesCollector;
use crate::interface::{AdminInterface, UpgradeableContract};

#[rule]
pub fn sanity(e: Env) {
    let address = nondet_address();
    let fee_collector = FeesCollector::init_admin(e, address);
    cvlr_assert!(false);
}