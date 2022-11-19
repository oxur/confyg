use serde_derive::Deserialize;
use confyg::Confygery;
use confyg::env;

const CFG: &str = r#"
env = "staging"

[servers]
platform = "GCP"

[servers_db]
host = "1.2.3.4"
name = "db"
user = "alice"
max_conns = "500"
"#;

#[derive(Debug, Default, Deserialize)]
struct Config {
    env: String,
    servers: Servers,
    servers_db: ServersDB,
}

#[derive(Debug, Default, Deserialize)]
struct Servers {
    platform: String,
}

#[derive(Debug, Default, Deserialize)]
struct ServersDB {
    host: String,
    name: String,
    user: String,
    max_conns: String,
}

fn main() {
    let opts = env::Options{
        top_level: "cfyg".to_string(),
        sections: vec![
            "servers".to_string(),
            "servers_db".to_string(),
        ],
    };
    let cfg: Config = Confygery::new()
        .use_env(opts)
        .add_str(CFG)
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
