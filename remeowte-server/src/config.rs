use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub(crate) struct KeyPair{
    pub(crate) ak: String,
    pub(crate) sk: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) key_pairs: Vec<KeyPair>,
}

pub(crate) fn read_config_from_file(file_path: &str) -> Result<Config, Box<dyn Error>> {
    // Open in read-only mode.
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = serde_json::from_str(&contents)?;

    Ok(config)
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_serde() {
        let res = read_config_from_file("./conf/keys.json").unwrap();
        assert_eq!(res.key_pairs.len(), 1);
        assert_eq!(res.key_pairs[0].ak, "test_ak");
        assert_eq!(res.key_pairs[0].sk, "test_sk");
    }
}