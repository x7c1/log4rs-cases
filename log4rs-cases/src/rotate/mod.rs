use std::str::FromStr;

use log::{Level, LevelFilter};
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

use crate::CasesResult;

pub struct Setting {
    pub appender_name: String,
    pub file_pattern: String,
    pub file_path: String,
    pub log_level: String,
    pub limit_size_kb: u64,
    pub limit_file_count: u32,
}

impl Setting {
    pub fn configure(&self) -> CasesResult<Config> {
        let config = Config::builder()
            .appender(self.create_appender()?)
            .build(self.create_root()?)?;

        Ok(config)
    }
    fn create_appender(&self) -> CasesResult<Appender> {
        let file_appender = Box::new(self.create_rolling_file_appender()?);
        let appender = Appender::builder().build(self.appender_name.as_str(), file_appender);
        Ok(appender)
    }
    fn create_root(&self) -> CasesResult<Root> {
        let root = Root::builder()
            .appender(self.appender_name.as_str())
            .build(self.get_level_filter()?);

        Ok(root)
    }
    fn create_rolling_file_appender(&self) -> CasesResult<RollingFileAppender> {
        let appender = RollingFileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} [{l}] {M} {m}{n}")))
            .build(&self.file_path, Box::new(self.create_policy()))?;

        Ok(appender)
    }
    fn create_policy(&self) -> CompoundPolicy {
        let trigger = SizeTrigger::new(self.limit_size_kb * 1024);
        let roller = FixedWindowRoller::builder()
            .build(&self.file_pattern, self.limit_file_count)
            .unwrap();

        CompoundPolicy::new(Box::new(trigger), Box::new(roller))
    }
    fn get_level_filter(&self) -> CasesResult<LevelFilter> {
        let filter = Level::from_str(&self.log_level)?.to_level_filter();
        Ok(filter)
    }
}
