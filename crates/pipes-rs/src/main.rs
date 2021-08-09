use config::Config;
use mimalloc::MiMalloc;
use pipes_rs::App;
use std::io;
use terminal::StdoutBackend;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> anyhow::Result<()> {
    let stdout = io::stdout();
    let backend = StdoutBackend::new(stdout.lock());
    let app = App::new(backend, Config::read()?)?;
    app.run()?;

    Ok(())
}
