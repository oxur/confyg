use confyg::Confygery;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Config {
    env: String,
    servers: Servers,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Servers {
    platform: String,
    db: DB,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct DB {
    host: String,
    name: String,
    user: String,
    max_conns: i16,
}

fn main() {
    let cfg: Config = match Confygery::new()
        .add_file("examples/confs/testing-dotted.toml")
        .build()
    {
        Ok(x) => x,
        Err(e) => panic!("{e}"),
    };
    println!("Deploy env: {}", cfg.env);
    println!("Servers platform: {}", cfg.servers.platform);
    println!("DB host: {}", cfg.servers.db.host);
    println!("DB name: {}", cfg.servers.db.name);
    println!("DB user: {}", cfg.servers.db.user);
    println!("DB max connections: {}", cfg.servers.db.max_conns);
}
