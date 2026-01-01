use crate::models::enums::Platform;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KReq {
    pub tracked_accounts: HashMap<String, Platform>,
}
