use carpenter::ConfigManager;

#[derive(ConfigManager, PartialEq, Debug)]
struct Config {
    _a: i32,
    _b: bool,
    _c: String,
}

fn main() {
    let test_config_factory = Config::create(
        "meloencoding", 
        "config-rs-test",
        "test.bin"
    );

    let sample_config = Config {
        _a: 400,
        _b: true,
        _c: String::from("Hey"),
    };
    test_config_factory.save(&sample_config);

    assert_eq!(sample_config, test_config_factory.read());
}
