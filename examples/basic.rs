use agent_hashmap::config::Config;
use agent_hashmap::store::Store;
use agent_hashmap::types::{Key, ProjectName, Value};

fn main() -> agent_hashmap::Result<()> {
    let cwd = std::env::current_dir()?;
    let config = Config::for_project_root(ProjectName::new("example-basic")?, cwd)?;

    let mut store = Store::open(config)?;
    let key = Key::new("agent/codex/current_task")?;
    let value = Value::new("implement local index")?;

    let _ = store.set(key.clone(), value)?;
    store.flush()?;

    if let Some(found) = store.get(&key) {
        println!("{} = {}", key, found);
    }

    Ok(())
}
