use agentmem::config::Config;
use agentmem::types::ProjectName;

fn main() -> agentmem::Result<()> {
    let cwd = std::env::current_dir()?;
    let config = Config::for_project_root(ProjectName::new("example-config")?, cwd)?;

    let path = agentmem::config::resolve_local_config_path()?;
    config.save(&path)?;

    println!("config written to {}", path.display());
    Ok(())
}
