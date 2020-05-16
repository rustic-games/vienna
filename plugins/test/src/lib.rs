vienna::load!();

fn init() -> Registration {
    Registration::new("test").state("pos_x", Value::from(0.0))
}

fn run(sdk: &mut Sdk) -> Result<()> {
    match sdk.get_mut("pos_x") {
        Some(pos_x) => {
            let v: f32 = pos_x.as_f64().unwrap_or(0.0) as f32;
            *pos_x = Value::from(v % 800.0 + 1.0);
        }
        None => bail!("oops!"),
    };

    Ok(())
}
