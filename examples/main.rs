use localizer_rs;

fn main() {
    let config: localizer_rs::Config = localizer_rs::Config::new("translations", "en");

    println!(
        "{:}",
        localizer_rs::t!(
            config,
            "error",
            "details" = "Something went wrong when trying to do stuff"
        )
    );
    println!(
        "{:}",
        localizer_rs::t!(config, "success", "balance" = "$10", "user" = "John Doe")
    );

    println!("{:}", localizer_rs::t!(config, "all"));
}
