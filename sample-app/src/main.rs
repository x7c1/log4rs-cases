#[macro_use]
extern crate log;

use log4rs_cases::hello;
use log4rs_cases::CasesResult;
use log4rs_cases::rotate::size::Setting;
use log4rs::config::Config;

fn main() {
    let config = create_config().unwrap();
    log4rs::init_config(config).unwrap();

    let greeting = hello("world");
    println!("{}", greeting);
    info!("info: {}", greeting);
}

pub fn create_config() -> CasesResult<Config> {
    let log_dir = "./sample";
    let setting = Setting {
        appender_name: "sample_log_appender".to_string(),
        file_pattern: format!("{}/{}", log_dir, "sample.{}.log"),
        file_path: format!("{}/{}", log_dir, "sample.log"),
        log_level: "info".to_string(),
        limit_size_kb: 1000,
        limit_file_count: 3,
    };
    let config = setting.configure()?;
    Ok(config)
}
