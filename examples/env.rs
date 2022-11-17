use serde_derive::Deserialize;
use confyg::Confygery;
use confyg::env::options::Options;

#[derive(Debug, Deserialize)]
struct Config {
    env: String,
    servers: Servers,
    servers_db: ServersDB,
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
    max_conns: String,
}

fn main() {
    // See the Makefile's 'demos' target for the ENV VARS that
    // get set for this demo.
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
    println!("DB host: {}", cfg.servers_db.host);
    println!("DB name: {}", cfg.servers_db.name);
    println!("DB user: {}", cfg.servers_db.user);
    println!("DB max connections: {}", cfg.servers_db.max_conns);
}
