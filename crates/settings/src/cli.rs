use clap::{CommandFactory, Parser, Subcommand};
use core::option::Option;
use std::{fs, path::PathBuf};

use crate::settings::Settings;

#[derive(Parser)]
#[command(name = "laks", version, about)]
pub struct Cli {
    /// Path to configuration file
    #[arg(id = "config", short = 'c', long = "config", global = true)]
    pub config_file: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {},
    /// Configuration management commands
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    // Only included in `cargo build` / `cargo run` (omitted in `--release`)
    #[cfg(debug_assertions)]
    /// Generate fake wheel packages
    Seed {
        /// Number of wheel packages to generate
        #[arg(default_value_t = 10)]
        count: usize,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Initialize a new configuration file with default values
    Init {
        /// Output file path (default: ./laks.toml)
        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,
    },
    /// Show the current configuration
    Show {},
}

pub enum CliResult {
    ShowHelp,
    RunServer(ResolvedSettings),
    InitConfig {
        output: PathBuf,
    },
    ShowConfig(ResolvedSettings),
    #[cfg(debug_assertions)]
    SeedDatabase {
        count: usize,
        resolved: ResolvedSettings,
    },
}

pub fn parse_cli() -> CliResult {
    let cli = Cli::parse();

    let default = ResolvedSettings {
        settings: Settings::default(),
    };

    match cli.command {
        Some(Commands::Start {}) => match cli.config_file {
            Some(config_file) => {
                let content = fs::read_to_string(config_file).unwrap();
                let toml: Settings = toml::from_str(&content).expect("lol");
                CliResult::RunServer(ResolvedSettings { settings: toml })
            }
            None => CliResult::RunServer(default),
        },
        Some(Commands::Config {
            command: ConfigCommands::Init { output },
        }) => CliResult::InitConfig {
            output: output.unwrap_or_else(|| PathBuf::from("laks.toml")),
        },
        Some(Commands::Config {
            command: ConfigCommands::Show {},
        }) => match cli.config_file {
            Some(config_file) => {
                let content = fs::read_to_string(config_file).unwrap();
                let toml: Settings = toml::from_str(&content).expect("lol");
                CliResult::ShowConfig(ResolvedSettings { settings: toml })
            }
            None => CliResult::ShowConfig(default),
        },
        #[cfg(debug_assertions)]
        Some(Commands::Seed { count }) => {
            let resolved = match cli.config_file {
                Some(config_file) => {
                    let content = fs::read_to_string(config_file)
                        .unwrap_or_else(|error| panic!("failed to read config: {error}"));
                    let settings = toml::from_str(&content)
                        .unwrap_or_else(|error| panic!("failed to parse config: {error}"));
                    ResolvedSettings { settings }
                }
                None => default,
            };
            CliResult::SeedDatabase { count, resolved }
        }

        None => {
            Cli::command().print_help().ok();
            CliResult::ShowHelp
        }
    }
}

pub struct ResolvedSettings {
    pub settings: Settings,
}
