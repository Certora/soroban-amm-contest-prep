use soroban_sdk::Env;

use cvlr::asserts::{cvlr_assert, cvlr_assume};
use cvlr_soroban_derive::rule;
use cvlr_soroban::nondet_address;
use cvlr::clog;

pub use crate::contract::FeesCollector;
use access_control::role::Role;
use access_control::management::SingleAddressManagementTrait;
use access_control::access::AccessControlTrait;
use access_control::emergency::{get_emergency_mode, set_emergency_mode};

use crate::interface::{AdminInterface, UpgradeableContract};

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

#[rule]
pub fn only_emergency_admin_sets_emergency_mode(e: Env) {
    let address = nondet_address();
    let value: bool = cvlr::nondet();
    cvlr_assume!(unsafe{!ACCESS_CONTROL.clone().unwrap().address_has_role(&address, &Role::EmergencyAdmin)});
    FeesCollector::set_emergency_mode(e, address, value);
    cvlr_assert!(false); // should not reach and therefore should pass
}

#[rule]
pub fn set_emergency_mode_success(e: Env) {
    let value: bool = cvlr::nondet();
    access_control::emergency::set_emergency_mode(&e, &value);
    cvlr_assert!(access_control::emergency::get_emergency_mode(&e) == false);
}