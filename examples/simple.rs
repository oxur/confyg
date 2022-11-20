use serde_derive::Deserialize;
// use confyg::Confygery;
use config::{Config, File, FileFormat};

static CFG: &str = r#"
[default]
env = "prod"

[servers]
platform = "GCP"

[servers.db]
host = "1.2.3.4"
name = "db"
user = "alice"
max_conns = 500
"#;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Default {
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
    // let cfg: Config = Confygery::new()
    //     .add_str(CFG)
    //     .build()
    //     .unwrap();
    let cfg = Config::builder()
        .add_source(File::from_str(CFG, FileFormat::Toml))
        .build()
        .unwrap();

    println!("Config: {:?}", cfg);
    // println!("Deploy env: {}", cfg.default.env);
    // println!("Servers platform: {}", cfg.servers.platform);
    // println!("DB host: {}", cfg.servers.db.host);
    // println!("DB name: {}", cfg.servers.db.name);
    // println!("DB user: {}", cfg.servers.db.user);
    // println!("DB max connections: {}", cfg.servers.db.max_conns);
    println!("Deploy env: {:?}", cfg.get::<String>("default.env").unwrap());
}
