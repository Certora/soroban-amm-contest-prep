use cvlr::clog;
use soroban_sdk::Env;

use cvlr::asserts::{cvlr_assert, cvlr_assume};
use cvlr_soroban_derive::rule;
use cvlr_soroban::nondet_address;

pub use crate::contract::FeesCollector;
use access_control::access::AccessControl;
use access_control::role::Role;
use access_control::management::SingleAddressManagementTrait;
use crate::interface::{AdminInterface, UpgradeableContract};

use crate::certora_specs::ACCESS_CONTROL;

#[rule]
pub fn init_admin_sets_admin(e: Env) {
    let address = nondet_address();
    let fee_collector = FeesCollector::init_admin(e, address.clone());
    let addr = unsafe { ACCESS_CONTROL.clone().unwrap().get_role(&Role::Admin) };
    cvlr_assert!(addr == address);
}