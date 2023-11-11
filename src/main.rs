use carpenter::ConfigManager;

// Create a struct and derive it with ConfigManager
#[derive(ConfigManager, PartialEq, Debug)]
struct Config {
    a: i32,
    b: bool,
    c: String,
}

fn main() -> Result<(), std::io::Error>{
    // Create config factory
    let config_factory = Config::init_config(
        "meloencoding", // Username
        "config-rs-test", // Application name
        "test.bin" // Config file name. File extention is optional
    );

    // You could create 20 of these if you want but make sure the 
    // config file name is different

    // To save your config
    let sample_config = Config {
        a: 400,
        b: true,
        c: String::from("Hey"),
    };

    config_factory.save(&sample_config)?;

    // To read the saved config
    assert_eq!(sample_config, config_factory.read()?);
    _test().unwrap();
    Ok(())
}


fn _test() -> Result<(), ()> {
    Ok(())
}