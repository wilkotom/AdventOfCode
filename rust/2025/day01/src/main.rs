use aochelpers::get_daily_input;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(1,2025)?;

    Ok(())
}


#[cfg(test)]
mod tests {


}