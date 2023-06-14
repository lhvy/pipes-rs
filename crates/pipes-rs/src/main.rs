use mimalloc::MiMalloc;
use pipes_rs::{App, Config};
use std::io::{self, Write};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> anyhow::Result<()> {
    let config = Config::read()?;

    if config.license {
        let mut stdout = io::stdout();
        stdout.write_all(b"pipes-rs is licensed under the Blue Oak Model License 1.0.0,\nthe text of which you will find below.\n\n")?;
        stdout.write_all(include_bytes!("../../../LICENSE.md"))?;
        stdout.flush()?;
        return Ok(());
    }

    let app = App::new(config)?;
    app.run()?;

    Ok(())
}
