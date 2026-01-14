use confyg::env;
use confyg::Confygery;
use serde_derive::Deserialize;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // See the Makefile's 'demos' target for the ENV VARS that
    // get set for this demo.
    let mut opts = env::Options::with_top_level("cfyg");
    opts.add_section("servers").add_section("servers_db");
    let cfg: Config = Confygery::new()?.add_env(opts)?.build()?;
    println!("Deploy env: {}", cfg.env);
    println!("Servers platform: {}", cfg.servers.platform);
    println!("DB host: {}", cfg.servers_db.host);
    println!("DB name: {}", cfg.servers_db.name);
    println!("DB user: {}", cfg.servers_db.user);
    println!("DB max connections: {}", cfg.servers_db.max_conns);
    Ok(())
}
