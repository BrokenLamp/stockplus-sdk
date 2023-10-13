use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    pub struct Permissions: u64 {
        const ADMIN = 1 << 0;
        const MANAGE_ITEMS = 1 << 1;
        const MANAGE_EXPENSES = 1 << 2;
        const MANAGE_ORDERS = 1 << 3;
        const VIEW_REPORTS = 1 << 4;
    }
}
