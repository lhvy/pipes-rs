use mimalloc::MiMalloc;
use pipes_rs::{App, Config};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> anyhow::Result<()> {
    let config = Config::read()?;
    let app = App::new(config)?;
    app.run()?;

    Ok(())
}
