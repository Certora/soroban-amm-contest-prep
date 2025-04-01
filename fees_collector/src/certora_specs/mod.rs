pub mod fee_collector_rules;

use access_control::access::AccessControl;
pub(crate) static mut ACCESS_CONTROL: Option<AccessControl> = None;