use econf::LoadEnv;

#[derive(Debug, strum::EnumString, LoadEnv)]
#[strum(serialize_all = "kebab-case")]
enum AuthMode {
    ApiKey,
    BasicAuth,
    #[strum(ascii_case_insensitive)]
    BearerToken,
    #[strum(serialize = "oauth", serialize = "OAuth")]
    OAuth,
    JWT,
}

#[derive(Debug, LoadEnv)]
struct Config {
    auth_mode: AuthMode,
    data: String,
    passwd: String,
}

fn main() {
    simple_logger::init().unwrap();

    let c = Config {
        auth_mode: AuthMode::ApiKey,
        data: "foo".into(),
        passwd: "bar".into(),
    };
    println!("Before loading env: {c:?}");

    let c = econf::load(c, "app");
    println!("After loading env: {c:?}");
}
