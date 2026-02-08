use std::collections::HashMap;

use once_cell::sync::Lazy;

pub mod types;
pub use types::*;

pub mod auth;
pub use auth::*;


pub static LANGUAGE_ID_MAP: Lazy<HashMap<&'static str, i16>> = Lazy::new(|| {
    HashMap::from([
        ("c++", 54),
        ("javascript", 63),
        ("rust", 73)
    ])
});