use std::env;

const KEY_PORT: &str = "PORT";

pub fn env_port_number() -> u16 {
    match env::var(KEY_PORT) {
        Ok(val) => val.parse().unwrap(),
        Err(_e) => 9090,
    }
}
