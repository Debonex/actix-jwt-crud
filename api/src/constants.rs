pub const DEFAULT_RUST_LOG: &str = "info,sqlx=off";
pub const DEFAULT_PORT: u16 = 7878;
pub const TOKEN_VALID_TIME: usize = 7 * 24 * 60 * 60;

// secret salt
pub const TOKEN_SECRET: &[u8; 16] = include_bytes!("../.secret");
pub const ARGON2_SALT: &[u8] = b"argon2_salt";
