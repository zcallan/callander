use dotenv::dotenv;
use std::env;

pub fn init_env() {
    dotenv().ok().expect("Env init error");
}

pub fn is_dev() -> bool {
    let env = env::var("RUN_ENV").expect("Runtime env not set");
    env == "dev"
}

pub fn get_host_port() -> (String, u16) {
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT")
        .expect("Port not set")
        .parse::<u16>()
        .unwrap();

    (host, port)
}

pub fn get_app_name() -> String {
    env::var("APP_NAME").unwrap_or("codefee-works-api".into())
}
