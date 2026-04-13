use agent_hashmap::config::Config;
use agent_hashmap::types::ProjectName;

fn main() -> agent_hashmap::Result<()> {
    let cwd = std::env::current_dir()?;
    let config = Config::for_project_root(ProjectName::new("example-config")?, cwd)?;

    let path = agent_hashmap::config::resolve_local_config_path()?;
    config.save(&path)?;

    println!("config written to {}", path.display());
    Ok(())
}
