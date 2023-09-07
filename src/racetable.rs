use serde::Deserialize;

use crate::circuit;
use crate::race_result;

#[derive(Deserialize, Debug, Clone)]
pub struct RaceTable {
    pub season: String,
    #[serde(rename = "Races")]
    pub races: Vec<Race>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub round: String,
    pub race_name: String,
    pub date: String,
    #[serde(rename = "Circuit")]
    pub circuit: circuit::Circuit,
    #[serde(rename = "Results")]
    #[serde(default)]
    pub race_results: Vec<Option<race_result::RaceResult>>,
}
