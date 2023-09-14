use localizer_rs;

fn main() {
    let config: localizer_rs::Config = localizer_rs::Config::new("translations", "en");

    println!(
        "{:}",
        config.t(
            "error",
            vec![("details", "Something went wrong when trying to do stuff")]
        )
    );
    println!(
        "{:}",
        config.t("success", vec![("balance", "$10"), ("user", "John Doe")])
    );

    println!("{:}", config.t("all", vec![]));
}
