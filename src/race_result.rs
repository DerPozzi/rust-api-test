use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

use crate::racetable::RaceTable;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RaceResult {
    #[serde(rename = "Drivers")]
    pub drivers: HashMap<String, Vec<Driver>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Driver {
    code: String,
    given_name: String,
    family_name: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    #[serde(rename = "RaceTable")]
    race_table: RaceTable,
}

pub async fn get_race_results(season: u32, race: u32) -> Result<Response, Error> {
    let requested_url = format!("http://ergast.com/api/f1/{}/{}/results.json", season, race);
    let resp = reqwest::get(&requested_url)
        .await?
        .json::<HashMap<String, Response>>()
        .await?
        .get("MRData")
        .expect("");
    // let total_races = &resp.total;
    let race_table = &resp.race_table;
    // let race = &race_table.races;

    Ok(Response {
        race_table: race_table,
    })
}
