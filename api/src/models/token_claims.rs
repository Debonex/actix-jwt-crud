use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: i32,
    pub iat: i64,
    pub exp: i64,
}
