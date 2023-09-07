use reqwest::Error;
use tokio;

mod circuit;
mod race_result;
mod racetable;
mod season;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let year = 2022;
    season::display_races_in_season(year).await?;

    Ok(())
}
