use soroban_sdk::Address;

use access_control::management::SingleAddressManagementTrait;
use access_control::role::Role;

use crate::certora_specs::ACCESS_CONTROL;

pub fn get_role_address() -> Address {
    let acc_ctrl = unsafe { &mut *&raw mut ACCESS_CONTROL };
    return acc_ctrl.as_ref().unwrap().get_role(&Role::Admin);
}
