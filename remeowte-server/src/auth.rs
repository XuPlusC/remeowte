use std::collections::HashMap;
use lazy_static::lazy_static;

const DEFAULT_AK: &str = "cococat";
const DEFAULT_SK: &str = "233";
const DEBUG_RELEASE_FLAG: bool = cfg!(debug_assertions);

lazy_static!{
    static ref AUTH_MAP: HashMap<String, String> = {
        let mut m = HashMap::new();
        if DEBUG_RELEASE_FLAG {
            m.insert(DEFAULT_AK.to_string(), DEFAULT_SK.to_string());
        }
        m
    };
}
