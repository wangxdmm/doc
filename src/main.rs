use std::error::Error;

use doc::cli::run;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(run()?)
}
