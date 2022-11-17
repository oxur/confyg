use serde_derive::Deserialize;
use confyg::Confygery;
use confyg::env::options::Options;

#[derive(Debug, Deserialize)]
struct Config {
    env: String,
    servers: Servers,
    serversdb: ServersDB,
}

#[derive(Debug, Deserialize)]
struct Servers {
    platform: String,
}

#[derive(Debug, Deserialize)]
struct ServersDB {
    host: String,
    name: String,
    user: String,
    max_conns: i16,
}

fn main() {
    let opts = Options{
        top_level: "cfyg".to_string(),
        sections: vec![
            "servers".to_string(),
            "servers_db".to_string(),
        ],
    };
    let cfg: Config = Confygery::new()
        .use_env(opts)
        .build()
        .unwrap();
    println!("toml: {:?}", cfg);
    println!("Deploy env: {}", cfg.env);
    println!("Servers platform: {}", cfg.servers.platform);
    println!("DB host: {}", cfg.serversdb.host);
    println!("DB name: {}", cfg.serversdb.name);
    println!("DB user: {}", cfg.serversdb.user);
    println!("DB max connections: {}", cfg.serversdb.max_conns);
}
