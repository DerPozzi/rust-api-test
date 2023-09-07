use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;

use reqwest::Error;

use crate::race_result::get_race_results;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct JsonResponse {
    total: String,
}

pub async fn display_races_in_season(season: u32) -> Result<(), Error> {
    let total_requested_url = format!("http://ergast.com/api/f1/{}.json", &season);
    let resp = reqwest::get(&total_requested_url)
        .await?
        .json::<HashMap<String, JsonResponse>>()
        .await?;
    let json_resp = resp.get("MRData").expect("");

    let total = json_resp.total.parse::<u32>().unwrap();

    println!("F1 season of {} - Total races: {}", season, &total);
    println!("Races held in the season:");

    for i in 0..total {
        let results = get_race_results(season, i + 1).await?;
        let race = &results.race_table.races;
        for race in race {
            let circuit = &race.circuit;

            println!(
                "Race {}{} was the {}",
                "Nr.".blue(),
                race.round.blue(),
                race.race_name
            );
            println!(
                "\t- {}, {} -> {}",
                circuit.location.country,
                circuit.location.locality,
                circuit.circuit_name.bright_red()
            );

            println!("\t- Date: {}", race.date);

            if let Some(results) = &race.race_results[0] {
                let winner = &results.driver;
                println!(
                    "\t- Winner: {} -> {} {}",
                    winner.code, winner.given_name, winner.family_name
                );
            } else {
                println!("No Results available");
            }

            println!();
        }
    }

    Ok(())
}
