mod ramp;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    ramp::begin_cli()?;

    Ok(())
}
