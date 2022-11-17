use merge::Merge;
use serde_derive::Deserialize;
use confyg::Confygery;

const CFG: &str = r#"
env = "prod"

[servers]
platform = "GCP"

[servers.db]
host = "1.2.3.4"
name = "db"
user = "alice"
max_conns = 500
"#;

#[derive(Debug, Default, Deserialize, Merge, StructOpt)]
#[serde(default)]
struct Config {
    env: String,
    servers: Servers,
}

#[derive(Debug, Default, Deserialize, Merge, StructOpt)]
#[serde(default)]
struct Servers {
    platform: String,
    db: DB,
}

#[derive(Debug, Default, Deserialize, Merge, StructOpt)]
#[serde(default)]
struct DB {
    host: String,
    name: String,
    user: String,
    max_conns: i16,
}

fn main() {
    let cfg: Config = Confygery::new()
        .add_str(CFG)
        .build()
        .unwrap();
    println!("Deploy env: {}", cfg.env);
    println!("Servers platform: {}", cfg.servers.platform);
    println!("DB host: {}", cfg.servers.db.host);
    println!("DB name: {}", cfg.servers.db.name);
    println!("DB user: {}", cfg.servers.db.user);
    println!("DB max connections: {}", cfg.servers.db.max_conns);
}
