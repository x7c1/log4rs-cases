#[macro_use]
extern crate log;

use log4rs::config::Config;
use log4rs_cases;
use log4rs_cases::rotate;

fn main() {
    let config = create_config().unwrap();
    log4rs::init_config(config).unwrap();

    let greeting = "hello, world!";
    println!("{}", greeting);
    debug!("debug: {}", greeting);
    info!("info: {}", greeting);
    warn!("warn: {}", greeting);
    error!("error: {}", greeting);
}

pub fn create_config() -> log4rs_cases::Result<Config> {
    let setting = rotate::size::SettingBuilder::new()
        .file_path("sample/sample.log")
        .file_pattern("backup/sample.{}.log")
        .build();

    let config = setting.configure()?;
    Ok(config)
}
