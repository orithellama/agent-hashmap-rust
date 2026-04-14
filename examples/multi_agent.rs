use agentmem::config::Config;
use agentmem::store::Store;
use agentmem::types::{Key, ProjectName, Value};

fn main() -> agentmem::Result<()> {
    let cwd = std::env::current_dir()?;
    let config = Config::for_project_root(ProjectName::new("example-multi-agent")?, cwd)?;

    let mut store = Store::open(config)?;

    let _ = store.set(
        Key::new("agent/claude/current_task")?,
        Value::new("review architecture")?,
    )?;
    let _ = store.set(
        Key::new("agent/codex/current_task")?,
        Value::new("implement index query")?,
    )?;
    store.flush()?;

    for entry in store.entries() {
        println!("{} = {}", entry.key, entry.value);
    }

    Ok(())
}
