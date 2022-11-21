use serde_derive::{Deserialize, Serialize};
use confyg::Confygery;

#[derive(Debug, Deserialize, Serialize)]
#[allow(unused)]
struct Config {
    env: String,
    servers: Servers,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(unused)]
struct Servers {
    platform: String,
    db: DB,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(unused)]
struct DB {
    host: String,
    name: String,
    user: String,
    max_conns: i16,
}

fn main() {
    let defaults = Config{
        env: "production".to_string(),
        servers: Servers{
            platform: "data centre".to_string(),
            db: DB {
                host: "5.0.9.9".to_string(),
                name: "megadb".to_string(),
                user: "dave".to_string(),
                max_conns: 250,
            }
        }
    };
    let cfg: Config = Confygery::new()
        .add_file("examples/confs/common-dotted.toml")
        .add_struct(&defaults)
        .build()
        .unwrap();
    println!("Deploy env: {}", cfg.env);
    println!("Servers platform: {}", cfg.servers.platform);
    println!("DB host: {}", cfg.servers.db.host);
    println!("DB name: {}", cfg.servers.db.name);
    println!("DB user: {}", cfg.servers.db.user);
    println!("DB max connections: {}", cfg.servers.db.max_conns);
}
