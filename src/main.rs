use anyhow::Result;
use tailwind_lsp_server::start_lsp;
use structured_logger::{json::new_writer, Builder};


fn main() -> Result<()> {
    Builder::with_level("DEBUG")
        .with_target_writer("*", new_writer(std::io::stderr()))
        .init();
    start_lsp()?;
    Ok(())
}

