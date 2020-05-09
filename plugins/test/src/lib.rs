vienna::load!();

fn init() -> Registration {
    Registration::new("test")
}

fn run() -> Result<()> {
    bail!("woops!")
}
