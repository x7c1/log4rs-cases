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
    pub log_level: Level,
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
        let filter = self.log_level.to_level_filter();
        Ok(filter)
    }
}

use std::marker::PhantomData;
pub struct Empty;
pub struct Filled;

/// see also:
///
/// * https://github.com/rust-unofficial/patterns/blob/master/patterns/builder.md
/// * https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
/// * https://keens.github.io/blog/2017/02/09/rustnochottoyarisuginabuilderpata_n/
pub struct SettingBuilder<Pattern, Path> {
    file_pattern: Option<String>,
    file_pattern_state: PhantomData<Pattern>,
    file_path: Option<String>,
    file_path_state: PhantomData<Path>,
    appender_name: Option<String>,
    log_level: Option<Level>,
    limit_size_kb: Option<u64>,
    limit_file_count: Option<u32>,
}

impl SettingBuilder<Empty, Empty> {
    pub fn new() -> Self {
        SettingBuilder {
            file_pattern: None,
            file_pattern_state: PhantomData,
            file_path: None,
            file_path_state: PhantomData,
            appender_name: None,
            log_level: None,
            limit_size_kb: None,
            limit_file_count: None,
        }
    }
}

impl<Pattern> SettingBuilder<Pattern, Empty> {
    pub fn file_path<A: Into<String>>(self, path: A) -> SettingBuilder<Pattern, Filled> {
        SettingBuilder {
            file_pattern: self.file_pattern,
            file_pattern_state: self.file_pattern_state,
            file_path: Some(path.into()),
            file_path_state: PhantomData,
            appender_name: self.appender_name,
            log_level: self.log_level,
            limit_size_kb: self.limit_size_kb,
            limit_file_count: self.limit_file_count,
        }
    }
}

impl<Path> SettingBuilder<Empty, Path> {
    pub fn file_pattern<A: Into<String>>(self, pattern: A) -> SettingBuilder<Filled, Path> {
        SettingBuilder {
            file_pattern: Some(pattern.into()),
            file_pattern_state: PhantomData,
            file_path: self.file_path,
            file_path_state: self.file_path_state,
            appender_name: self.appender_name,
            log_level: self.log_level,
            limit_size_kb: self.limit_size_kb,
            limit_file_count: self.limit_file_count,
        }
    }
}

impl SettingBuilder<Filled, Filled> {
    pub fn build(self) -> Setting {
        Setting {
            appender_name: self.appender_name.unwrap_or("default".to_string()),
            file_pattern: self.file_pattern.unwrap(),
            file_path: self.file_path.unwrap(),
            log_level: self.log_level.unwrap_or(Level::Debug),
            limit_size_kb: self.limit_size_kb.unwrap_or(1000),
            limit_file_count: self.limit_file_count.unwrap_or(3),
        }
    }
}

impl<Pattern, Path> SettingBuilder<Pattern, Path> {
    pub fn log_level<A: Into<String>>(mut self, level: A) -> CasesResult<Self> {
        let log_level = Level::from_str(level.into().as_str())?;
        self.log_level = Some(log_level);
        Ok(self)
    }

    pub fn limit_size_kb(mut self, size_kb: u64) -> Self {
        self.limit_size_kb = Some(size_kb);
        self
    }

    pub fn limit_file_count(mut self, count: u32) -> Self {
        self.limit_file_count = Some(count);
        self
    }
}
