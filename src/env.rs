use std::env;
// List of env variables used by this service
/// Listen on server port eg. 9090
const KEY_PORT: &str = "PORT";
/// Database Source Name
///  Example:
/// mysql://root:root@localhost:3306/db
/// postgres://root:root@localhost:5432/db
/// sqlite:./sqlite.db?mode=rwc
/// sqlite::memory:
const KEY_DATABASE_URL: &str = "DATABASE_URL";

pub fn env_port_number() -> u16 {
    match env::var(KEY_PORT) {
        Ok(val) => val.parse().unwrap(),
        Err(_e) => 9090,
    }
}

pub fn env_database_url() -> String{
    match  env::var(KEY_DATABASE_URL) {
        Ok(val) => val,
        Err(_e) => "sqlite::memory:".to_string()
    }
}
