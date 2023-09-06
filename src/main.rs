use std::collections::HashMap;

use reqwest::Error;
use serde::Deserialize;
use tokio;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonResponse {
    total: String,
    #[serde(rename = "RaceTable")]
    race_table: RaceTable,
}

#[derive(Deserialize, Debug)]
struct RaceTable {
    season: String,
    #[serde(rename = "Races")]
    races: Vec<Race>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Race {
    round: String,
    race_name: String,
    date: String,
    #[serde(rename = "Circuit")]
    circuit: Circuit,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Circuit {
    circuit_name: String,
    #[serde(rename = "Location")]
    location: Location,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Location {
    lat: String,
    long: String,
    locality: String,
    country: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let year = 2023;
    let requested_url = format!("http://ergast.com/api/f1/{}.json", year);
    println!("Requested URL: {}", requested_url);
    let resp = reqwest::get(&requested_url)
        .await?
        .json::<HashMap<String, JsonResponse>>()
        .await?;

    let json_resp = resp.get("MRData").expect("");
    let total_races = &json_resp.total;
    let race_table = &json_resp.race_table;
    let races = &race_table.races;
    println!(
        "F1 season of {} - Total races: {}",
        race_table.season, total_races
    );
    for race in races {
        let circuit = &race.circuit;
        println!("Race Nr.{} was the {}", race.round, race.race_name);
        println!(
            "\t- {}, {} -> {}",
            circuit.location.country, circuit.location.locality, circuit.circuit_name
        );
        println!("\t- Date: {}", race.date)
    }

    Ok(())
}
