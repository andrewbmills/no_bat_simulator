use no_bat_simulator::{
    calculate_obp, convert_string_to_date, simplify_outcome_codes, simplify_pitch_codes,
    simulate_plate_appearance_no_bat,
};
use std::fs;
use std::path::Path;

use no_bat_simulator::{Date, PlateAppearance};

static DATA_DIR: &str = "C:/Users/Andrew/Documents/Coding/no_bat_simulator/data";

fn split_player_name_into_first_and_last(player_name: &String) -> (String, String) {
    let player_name_split = player_name.split(" ").collect::<Vec<&str>>();
    let player_firstname = player_name_split[0].to_owned();
    let player_lastname = player_name_split[1..].join(" ");
    (player_firstname, player_lastname)
}

fn get_player_id_from_file(player_name: &String, team_name: &String, year: i32) -> String {
    let (player_firstname, player_lastname) = split_player_name_into_first_and_last(player_name);

    // Open the .ROS file for the player's team and find the player's ID
    let path_text = format!(
        "{DATA_DIR}/{}eve/{team_name}{}.ROS", year, year
    );
    let path_in_roster = Path::new(&path_text);
    let contents_roster =
        fs::read_to_string(path_in_roster).expect("Something went wrong reading the file");
    let lines = contents_roster.split("\n");
    for line in lines {
        // Check if the line contains the player's name
        let line_data = line.split(",").collect::<Vec<&str>>();
        if line_data[1] == &player_lastname && line_data[2] == &player_firstname {
            // If so, return the player's ID
            return line_data[0].to_owned();
        }
    }
    // If the player's name was not found in the roster, return an empty string
    let blank_string = "";
    return blank_string.to_owned();
}

fn read_plate_appearances_from_file(player_name: &String, team_name: &String, year: i32) -> Vec<PlateAppearance> {
    let mut plate_appearances: Vec<PlateAppearance> = Vec::new();
    let player_id = get_player_id_from_file(player_name, team_name, year);

    // Open the .EVA file for the player's team
    let path_text = format!(
        "{DATA_DIR}/{}eve/{}{team_name}.EVN", year, year
    );
    let path_in = Path::new(&path_text);
    let contents = fs::read_to_string(path_in).expect("Something went wrong reading the file");
    // Split data by newline
    let lines = contents.split("\n");
    let mut last_game_date = Date::new(0, 0, 0);
    for line in lines {
        // Remove trailing newline character
        let mut line_trimmed = line.to_string();
        line_trimmed.pop();
        // Check if the line contains the player's ID
        let line_data = line_trimmed.split(",").collect::<Vec<&str>>();
        if line_data[0] == "info" && line_data[1] == "date" {
            last_game_date = convert_string_to_date(line_data[2]);
        }
        if line_data[0] == "play" && line_data[3] == player_id {
            // If so, create a plate appearance from the line and add it to the vector
            let pitches_complex = line_data[5].chars().collect::<Vec<char>>();
            let mut pitches: Vec<char> = Vec::new();
            for pitch_complex in pitches_complex {
                let pitch: char = simplify_pitch_codes(pitch_complex);
                if pitch == 'N' {
                    continue;
                }
                pitches.push(pitch);
            }
            // Parse the outcome of the plate appearance
            let outcome_complex = line_data[6].chars().collect::<Vec<char>>()[0];
            let outcome = simplify_outcome_codes(outcome_complex);

            if outcome == 'N' {
                // not a plate appearance
                continue;
            }

            let plate_appearance = PlateAppearance::new(
                last_game_date.clone(),
                player_name.to_owned(),
                outcome,
                pitches,
            );
            plate_appearances.push(plate_appearance);
        }
    }
    return plate_appearances;
}

fn read_oswing_and_zone_pct_from_file(player_name: &String, year: i32) -> (f32, f32) {
    let (player_firstname, player_lastname) = split_player_name_into_first_and_last(player_name);
    let path_text = format!("{DATA_DIR}/{}eve/{}_plate_discipline.csv", year, year);
    let path_in = Path::new(&path_text);
    let contents = fs::read_to_string(path_in).expect("Something went wrong reading the file");
    // Split data by newline
    let lines = contents.split("\n");
    for line in lines {
        // Remove trailing newline character
        let mut line_trimmed = line.to_string();
        line_trimmed.pop();
        // Check if the line contains the player's name
        let line_data = line_trimmed.split(", ").collect::<Vec<&str>>();
        if line_data[0] == player_firstname && line_data[1] == player_lastname {
            let mut oswing_pct_string = line_data[2].to_string();
            oswing_pct_string.pop(); // removed percent sign
            let oswing_pct = match oswing_pct_string.parse::<f32>() {
                Ok(oswing_pct) => oswing_pct,
                Err(e) => panic!(
                    "Error parsing oswing_pct string: {}, ({})",
                    oswing_pct_string, e
                ),
            };
            let mut zone_pct_string = line_data[3].to_string();
            zone_pct_string.pop(); // removed newline
            zone_pct_string.pop(); // removed percent sign
            let zone_pct = match zone_pct_string.parse::<f32>() {
                Ok(zone_pct) => zone_pct,
                Err(e) => panic!(
                    "Error parsing zone_pct string: {}, ({})",
                    zone_pct_string, e
                ),
            };
            return (oswing_pct, zone_pct);
        }
    }
    (-1.0, -1.0)
}

fn main() {
    // Parse the command line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut debug = false;
    if args.len() < 4 {
        panic!("Please provide a player name, ateam name, and a year as arguments");
    }
    else if args.len() > 4 {
        if args[4] == "debug" {
            println!("Debug mode enabled");
            debug = true;
        }
    }

    // Get the first argument passed to the program which should be the name of a player in the 2023 MLB season
    let player_name = &args[1];
    let team_name = &args[2];
    let year = args[3].parse::<i32>().unwrap();

    // Calculate the player's OBP for 2023
    let plate_appearances = read_plate_appearances_from_file(player_name, team_name, year);
    println!(
        "OBP for {} in {}: {}",
        player_name,
        year, 
        calculate_obp(&plate_appearances)
    );

    // Read the oswing_pct and zone_pct from the player's 2023 .EVA file
    let (oswing_pct, zone_pct) = read_oswing_and_zone_pct_from_file(player_name, year);

    // Resimulate all their 2023 at-bats as if they had no bat
    let mut plate_appearances_no_bat: Vec<PlateAppearance> = Vec::new();
    for appearance in &plate_appearances {
        plate_appearances_no_bat.push(simulate_plate_appearance_no_bat(
            appearance,
            oswing_pct,
            zone_pct,
        ));
    }

    // Calculate their OBP for 2023 again
    println!(
        "OBP for {} in {} without a bat: {}",
        player_name,
        year,
        calculate_obp(&plate_appearances_no_bat)
    );

    // All plate appearance with/without bat for debug
    if debug {
        for i in 0..plate_appearances.len() {
            println!(
                "{} ---> {}",
                plate_appearances[i],
                plate_appearances_no_bat[i]
            );
        }
    }
}
