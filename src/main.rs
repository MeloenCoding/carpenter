use carpenter::ConfigManager;

#[derive(ConfigManager, Default, Debug)]
struct Config {
    _a: i32,
    _b: bool,
    _c: bool,
    _d: String,
}

fn main() {
    let test_config_builder = Config::create(
        "meloencoding", 
        "config-rs-test",
        "test.bin"
    );

    let sample_config = Config {
        _a: 400,
        _b: true,
        _c: false,
        _d: String::from("Hey"),
    };
    test_config_builder.save(&sample_config);

    dbg!(test_config_builder.read());
}
