use reqwest::Error;
use serde::Deserialize;
use std::collections::HashMap;

use crate::racetable::RaceTable;
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RaceResult {
    #[serde(rename = "Driver")]
    pub driver: Driver,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    pub code: String,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "RaceTable")]
    pub race_table: RaceTable,
}

pub async fn get_race_results(season: u32, race: u32) -> Result<Response, Error> {
    let requested_url = format!("http://ergast.com/api/f1/{}/{}/results.json", season, race);
    let resp = reqwest::get(&requested_url)
        .await?
        .json::<HashMap<String, Response>>()
        .await?;

    let json_resp = resp.get("MRData").expect("");

    let race_table = &json_resp.race_table;

    Ok(Response {
        race_table: race_table.clone(),
    })
}
