use soroban_sdk::Address;

use access_control::management::SingleAddressManagementTrait;
use access_control::access::AccessControlTrait;
use access_control::role::Role;

use crate::certora_specs::ACCESS_CONTROL;

pub fn get_role_address() -> Address {
    let acc_ctrl = unsafe { &mut *&raw mut ACCESS_CONTROL };
    return acc_ctrl.as_ref().unwrap().get_role(&Role::Admin);
}

pub fn is_role(address: &Address, role: &Role) -> bool {
    let acc_ctrl = unsafe { &mut *&raw mut ACCESS_CONTROL };
    return acc_ctrl.as_ref().unwrap().address_has_role(&address, role)
}