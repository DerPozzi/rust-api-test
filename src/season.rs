use serde::Deserialize;
use std::collections::HashMap;

use reqwest::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonResponse {
    total: String,
}

pub async fn display_races_in_season(season: u32) -> Result<(), Error> {
    let total_requested_url = format!("http://ergast.com/api/f1/{}.json", season);
    let resp = reqwest::get(&total_requested_url)
        .await?
        .json::<HashMap<String, JsonResponse>>()
        .await?
        .get("MRData")
        .expect("");

    // println!(
    //     "F1 season of {} - Total races: {}",
    //     race_table.season, total_races
    // );
    // for race in races {
    //     let circuit = &race.circuit;
    //     println!("Race Nr.{} was the {}", race.round, race.race_name);
    //     println!(
    //         "\t- {}, {} -> {}",
    //         circuit.location.country, circuit.location.locality, circuit.circuit_name
    //     );
    //     println!("\t- Date: {}", race.date);
    //     if let Some(results) = &race.result {
    //         let winner = &results.drivers.get("0").expect("")[0];
    //         println!(
    //             "\t- Race Winner: {} - {} {}",
    //             winner.code, winner.given_name, winner.family_name
    //         );
    //     }
    // }

    Ok(())
}
