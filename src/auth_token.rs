use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthToken(pub String);

// impl Debug for AuthToken {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("AuthToken").field(&self.0.len()).finish()
//     }
// }
