vienna::load!();

fn init() -> Registration {
    Registration::new("test").write("pos_x", Float(0.0))
}

fn run(state: &mut State) -> Result<()> {
    match state.get_float_mut("pos_x") {
        Some(pos_x) => *pos_x = *pos_x % 800.0 + 1.0,
        None => bail!("oops!"),
    };

    Ok(())
}
