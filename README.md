# log4rs-cases

Modules to use log4rs easily.

## Usage

```rust
fn main() {

    // 1. setting to rotate log files by size.
    let setting = rotate::size::SettingBuilder::new()
        .file_path("sample/sample.log")
        .file_pattern("backup/sample.{}.log")
        .build();

    // 2. create log4rs Config.
    let config = setting.configure().unwrap();

    // 3. call init_config() as usual.
    log4rs::init_config(config).unwrap();

    let greeting = "hello, world!";

    debug!("debug: {}", greeting);
    info!("info: {}", greeting);
    warn!("warn: {}", greeting);
    error!("error: {}", greeting);
}
```

output:

```
$ tail -f sample/sample.log

2019-08-10T16:59:36.011987+09:00 [DEBUG] sample_app debug: hello, world!
2019-08-10T16:59:36.012626+09:00 [INFO] sample_app info: hello, world!
2019-08-10T16:59:36.012680+09:00 [WARN] sample_app warn: hello, world!
2019-08-10T16:59:36.012731+09:00 [ERROR] sample_app error: hello, world!
```
