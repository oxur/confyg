use serde_derive::Deserialize;
use confyg::Confygery;

#[derive(Debug, Default, Deserialize)]
#[allow(unused)]
struct Config {
    env: String,
    servers: Servers,
}

#[derive(Debug, Default, Deserialize)]
#[allow(unused)]
struct Servers {
    platform: String,
    db: DB,
}

#[derive(Debug, Default, Deserialize)]
#[allow(unused)]
struct DB {
    host: String,
    name: String,
    user: String,
    max_conns: i16,
}

fn main() {
    let empty = Config{
        .. Default::default()
    };
    let result = Confygery::new()
        .add_file("no-such-file.toml")
        .build::<Config>();
    let cfg = match result {
        Ok(x) => x,
        Err(_) => empty,
    };
    println!("Deploy env: {}", cfg.env);
    println!("Servers platform: {}", cfg.servers.platform);
    println!("DB host: {}", cfg.servers.db.host);
    println!("DB name: {}", cfg.servers.db.name);
    println!("DB user: {}", cfg.servers.db.user);
    println!("DB max connections: {}", cfg.servers.db.max_conns);
}
