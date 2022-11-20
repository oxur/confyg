use serde_derive::Deserialize;
use confyg::Confygery;
use confyg::conf;

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
    let opts = conf::Options{
        paths: vec![
            "./".to_string(),
            "../".to_string(),
            "examples".to_string(),
            "examples/confs".to_string(),
        ],
        .. Default::default()
    };
    let cfg: Config = Confygery::new()
        .with_opts(opts)
        .add_file("testing-dotted.toml")
        .build()
        .unwrap();
    println!("Config: {:?}", cfg);
    println!("Deploy env: {}", cfg.env);
    println!("Servers platform: {}", cfg.servers.platform);
    println!("DB host: {}", cfg.servers.db.host);
    println!("DB name: {}", cfg.servers.db.name);
    println!("DB user: {}", cfg.servers.db.user);
    println!("DB max connections: {}", cfg.servers.db.max_conns);
}
