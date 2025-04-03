use soroban_sdk::Env;

use cvlr::asserts::cvlr_assert;
use cvlr_soroban_derive::rule;
use cvlr_soroban::nondet_address;
use cvlr::clog;

pub use crate::contract::FeesCollector;
use access_control::role::Role;
use access_control::management::SingleAddressManagementTrait;
use crate::interface::AdminInterface;

use crate::certora_specs::ACCESS_CONTROL;

#[rule]
pub fn init_admin_sets_admin(e: Env) {
    let address = nondet_address();
    clog!(cvlr_soroban::Addr(&address));
    FeesCollector::init_admin(e, address.clone());
    let addr = unsafe { ACCESS_CONTROL.clone().unwrap().get_role(&Role::Admin) };
    clog!(cvlr_soroban::Addr(&addr));
    cvlr_assert!(addr == address);
}