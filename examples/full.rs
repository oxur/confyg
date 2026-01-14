use confyg::conf;
use confyg::env;
use confyg::Confygery;
use serde_derive::Deserialize;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conf_opts = conf::Options::default();
    conf_opts
        .add_path("./")
        .add_path("../")
        .add_path("examples")
        .add_path("examples/confs");

    let mut env_opts = env::Options::with_top_level("cfyg");
    env_opts.add_section("servers").add_section("servers_db");
    let cfg: Config = Confygery::new()?
        .with_opts(conf_opts)?
        .add_str(CFG)?
        .add_env(env_opts)?
        .add_file("common-under.toml")?
        .add_file("testing-under.toml")?
        .build()?;
    println!("Deploy env: {}", cfg.env);
    println!("Servers platform: {}", cfg.servers.platform);
    println!("DB host: {}", cfg.servers_db.host);
    println!("DB name: {}", cfg.servers_db.name);
    println!("DB user: {}", cfg.servers_db.user);
    println!("DB max connections: {}", cfg.servers_db.max_conns);
    Ok(())
}
