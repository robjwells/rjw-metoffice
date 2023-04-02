use std::{error::Error, fs::File};

use metoffice::Response;

fn main() -> Result<(), Box<dyn Error>>{
    {
        let file = File::open("shanklin.json")?;
        let result: Response = serde_json::from_reader(file)?;
        println!("{:#?}", result);
    }
    {
        let file = File::open("shanklin.json")?;
        let forecast = metoffice::site_forecast_from_reader(file)?;
        println!("{:#?}", forecast);
    }
    Ok(())
}
