use std::{
    collections::HashMap,
    sync::RwLock,
};
use lazy_static::lazy_static;
use tracing;
use once_cell::{self, sync::Lazy};

const DEFAULT_AK: &str = "cococat";
const DEFAULT_SK: &str = "233";
const DEBUG_FLAG: bool = cfg!(debug_assertions);
static AUTH_MAP: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    if DEBUG_FLAG {
        tracing::warn!("debug flag set, adding default AK SK.");
        m.insert(DEFAULT_AK.to_string(), DEFAULT_SK.to_string());
    }
    RwLock::new(m)
});

pub fn auth_key(ak: String, sk: String) -> bool {
    match AUTH_MAP.read().unwrap().get(&ak) {
        Some(v) => {
            *v == sk
        },
        None => false
    }
}

pub fn init(keys: HashMap<String, String>) {
    for (ak, sk) in keys.into_iter() {
        if let Some(old_sk) = AUTH_MAP.write().unwrap().insert(ak.clone(), sk.clone()) {
            tracing::warn!("exsisting key(ak:{}, sk:{}) has been updated to (ak:{}, sk:{})",
                ak, old_sk, ak, sk);
        }
    }
}

// TODO: make a reload api.

#[cfg(test)]
mod test{
    #[test]
    fn test() {
        
    }
}