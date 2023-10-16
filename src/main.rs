use clap::{Parser, Subcommand};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, write},
    process::Command,
    thread::sleep,
    time::Duration, env,
};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config_name: String,
    #[command(subcommand)]
    command: SubRoutines,
}

#[derive(Subcommand, Debug)]
enum SubRoutines {
    Init(Config),
    OneShot,
    Daemon {
        #[arg(short, long)]
        interval: u64,
    },
}

#[derive(Serialize, Deserialize, Parser, Debug, Clone)]
struct Config {
    #[arg(long)]
    username: String,
    #[arg(long)]
    hostname: String,
    #[arg(long)]
    port: Option<u16>,
    #[arg(long)]
    remote_path: String,
    #[arg(long)]
    local_path: String,
    #[serde(default)]
    #[arg(long)]
    excludes: Vec<String>,
}

impl Config {
    fn ssh_command(&self) -> String {
        format!("ssh -p {}", self.port.unwrap_or(22))
    }

    fn remote_arg(&self) -> String {
        format!("{}@{}:{}", self.username, self.hostname, self.remote_path)
    }

    fn exclude_args(&self) -> Vec<String> {
        self.excludes
            .iter()
            .map(|value| format!("--exclude={}", value))
            .collect::<Vec<_>>()
    }
}

fn main() {
    let Args {
        config_name,
        command,
    } = Args::parse();

    let basedirs = BaseDirs::new().unwrap();
    let target = basedirs.home_dir();
    let target = target
        .join(".rsync_ssh")
        .join(format!("{}.yml", config_name));

    if let SubRoutines::Init(config) = command {
        let config_content = serde_yaml::to_string(&config).unwrap();
        write(target, config_content).unwrap();
    } else if let SubRoutines::Daemon { interval } = command {
        let duration = Duration::from_secs(interval);
        loop {
            let command = env::current_exe().unwrap();
            let mut child = Command::new(command)
                .arg("-c")
                .arg(&config_name)
                .arg("one-shot")
                .spawn()
                .unwrap();
            child.wait().unwrap();
            println!("Finished. Next step will run {} seconds later", {interval});
            sleep(duration)
        }
    } else {
        let config = read_to_string(target).unwrap();
        let config: Config = serde_yaml::from_str(&config).unwrap();
        let mut child = Command::new("rsync")
            .arg("-avz")
            .arg("-e")
            .arg(config.ssh_command())
            .arg(config.remote_arg())
            .arg(&config.local_path)
            .arg("--progress")
            .args(config.exclude_args())
            .spawn()
            .unwrap();
        child.wait().unwrap();
    }
}
