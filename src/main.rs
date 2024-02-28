use std::error::Error;

mod zipper;

fn main() -> Result<(), Box<dyn Error>>{
    zipper::create_file()?;
    zipper::readfile()?;
    zipper::compress()?;
    zipper::extract()?;
    Ok(())
}


