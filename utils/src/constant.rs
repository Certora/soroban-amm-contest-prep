pub const DAY_IN_LEDGERS: u32 = 17280;
pub const WEEK_IN_LEDGERS: u32 = DAY_IN_LEDGERS * 7;
pub const MONTH_IN_LEDGERS: u32 = DAY_IN_LEDGERS * 30;

// Instance TTL
pub const MAX_INSTANCE_TTL: u32 = MONTH_IN_LEDGERS * 6;
pub const INSTANCE_TTL_THRESHOLD: u32 = MAX_INSTANCE_TTL - MONTH_IN_LEDGERS;

// Persistent TTL
pub const MAX_PERSISTENT_TTL: u32 = MONTH_IN_LEDGERS * 6;
pub const PERSISTENT_TTL_THRESHOLD: u32 = MAX_PERSISTENT_TTL - MONTH_IN_LEDGERS;

// Temporary TTL
pub const MAX_TEMPORARY_TTL: u32 = WEEK_IN_LEDGERS;
pub const TEMPORARY_TTL_THRESHOLD: u32 = MAX_TEMPORARY_TTL - DAY_IN_LEDGERS;
