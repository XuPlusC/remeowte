use clap::{Parser, Subcommand};
use std::{
    env,
    io::Seek,
    path::{Path, PathBuf},
};

const DEFUALT_CONFIG: &str = "config.json";
const DEFAULT_LOG_DIR: &str = "logs";

#[derive(Parser)]
#[command(name = "remeowte")]
#[command(author = "cococat; mail: https://github.com/XuPlusC")]
#[command(version = "1.0")]
#[command(about = "Remeowte: A remote command execution tool for server maintainance", long_about = None)]
pub(crate) struct Cli {
    /// Path for config file. If not set, will use 'config.json' in the binary path.
    #[arg(short, long, value_name = "FILE")]
    pub(crate) config: Option<PathBuf>,

    /// Directory of output log file. If not set, will use './logs/' in the binary path.
    #[arg(short, long, value_name = "FILE")]
    pub(crate) log_dir: Option<PathBuf>,

    /// Sets the level of verbosity
    #[clap(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

pub(crate) struct RuntimeParam {
    pub(crate) config_path: PathBuf,
    pub(crate) log_dir: PathBuf,
    pub(crate) verbose: bool,
}

impl Cli {
    fn concat_path(prefix_dir: &PathBuf, cli_field: &Option<PathBuf>, default: &str) -> PathBuf {
        let mut ret = prefix_dir.clone();
        let path = match cli_field {
            Some(path) => path,
            None => Path::new(default),
        };
        ret.push(path);
        ret
    }

    pub(crate) fn init() -> RuntimeParam {
        let runtime_dir = env::current_dir().unwrap();

        let cli = Cli::parse();

        let config_path = Cli::concat_path(&runtime_dir, &cli.config, DEFUALT_CONFIG);
        println!("[CLI] config path: {}", config_path.display());

        let log_dir = Cli::concat_path(&runtime_dir, &cli.log_dir, DEFAULT_LOG_DIR);
        println!("[CLI] log dir: {}", log_dir.display());

        RuntimeParam {
            config_path,
            log_dir,
            verbose: cli.verbose
        }
    }
}
