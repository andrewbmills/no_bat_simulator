use no_bat_simulator::{
    calculate_obp, convert_string_to_date, is_ball, is_ball_put_into_play_or_hit_by_pitch, is_foul,
    is_strike, simplify_outcome_codes, simplify_pitch_codes, simulate_plate_appearance_no_bat,
    Date,
};
use std::fs;
use std::path::Path;

use no_bat_simulator::PlateAppearance;

static DATA_DIR: &str = "data";

fn split_player_name_into_first_and_last(player_name: &String) -> (String, String) {
    let player_name_split = player_name.split(" ").collect::<Vec<&str>>();
    let player_firstname = player_name_split[0].to_owned();
    let player_lastname = player_name_split[1..].join(" ");
    (player_firstname, player_lastname)
}

fn get_player_id_from_file(player_name: &String, team_name: &String, year: i32) -> String {
    let (player_firstname, player_lastname) = split_player_name_into_first_and_last(player_name);

    // Open the .ROS file for the player's team and find the player's ID
    let path_text = format!("{DATA_DIR}/{}eve/{team_name}{}.ROS", year, year);
    let path_in_roster = Path::new(&path_text);
    let contents_roster =
        fs::read_to_string(path_in_roster).expect("Something went wrong reading the file");
    let lines = contents_roster.split("\n");
    for line in lines {
        // Check if the line contains the player's name
        let line_data = line.split(",").collect::<Vec<&str>>();
        if line_data.len() < 3 {
            continue;
        }
        if line_data[1] == &player_lastname && line_data[2] == &player_firstname {
            // If so, return the player's ID
            return line_data[0].to_owned();
        }
    }
    // If the player's name was not found in the roster, return an empty string
    let blank_string = "";
    return blank_string.to_owned();
}

fn read_plate_appearances_from_file(
    player_id: &String,
    player_name: &String,
    path: &Path,
) -> Vec<PlateAppearance> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading Retrosheet files");
    // Split data by newline
    let lines = contents.split("\n");
    let mut last_game_date = Date::new(0, 0, 0);
    let mut plate_appearances: Vec<PlateAppearance> = Vec::new();
    for line in lines {
        // Remove trailing newline character
        let mut line_trimmed = line.to_string();
        line_trimmed.pop();
        let line_data = line_trimmed.split(",").collect::<Vec<&str>>();
        if line_data[0] == "info" && line_data[1] == "date" {
            last_game_date = convert_string_to_date(line_data[2]);
        }
        // Check if the line contains the player's ID
        if line_data[0] == "play" && line_data[3] == player_id {
            // If so, create a plate appearance from the line and add it to the vector
            let pitches_complex = line_data[5].chars().collect::<Vec<char>>();
            let mut pitches: Vec<char> = Vec::new();
            let mut balls = 0;
            let mut strikes = 0;
            let mut outcome = 'N';
            for pitch in &pitches_complex {
                let pitch_simple: char = simplify_pitch_codes(pitch);
                if pitch_simple == 'N' {
                    continue;
                }
                pitches.push(pitch_simple);
                if is_ball(pitch) {
                    balls += 1;
                } else if is_strike(pitch) {
                    strikes += 1;
                } else if is_foul(pitch) {
                    strikes = std::cmp::min(strikes + 1, 2);
                } else if is_ball_put_into_play_or_hit_by_pitch(pitch) {
                    // If the pitch was put into play or hit the batter, the plate appearance is over
                    let outcome_complex = line_data[6].chars().collect::<Vec<char>>()[0];
                    outcome = simplify_outcome_codes(pitch, &outcome_complex);
                }
            }

            // Parse the outcome of the plate appearance
            if outcome == 'N' {
                if balls == 4 {
                    outcome = 'W';
                } else if strikes == 3 {
                    outcome = 'K';
                }
            }

            // println!("{:?} -> {:?}", pitches_complex, pitches);
            // println!("Outcome = {} from {}", outcome, line_data[6].to_owned());

            if outcome == 'N' {
                continue;
                // println!("Error: outcome not found for plate appearance");
            }

            let plate_appearance = PlateAppearance::new(
                last_game_date.clone(),
                player_name.to_owned(),
                outcome,
                pitches,
                line_data[6].to_owned(),
            );
            plate_appearances.push(plate_appearance);
        }
    }
    return plate_appearances;
}

fn read_in_plate_appearances(
    player_name: &String,
    team_name: &String,
    year: i32,
) -> Vec<PlateAppearance> {
    let mut plate_appearances: Vec<PlateAppearance> = Vec::new();
    let player_id = get_player_id_from_file(player_name, team_name, year);
    // Open the .EVA/.EVN files and grab all plate appearances for the player
    for dir in std::fs::read_dir(format!("{DATA_DIR}/{}eve/", year)).unwrap() {
        let path = dir.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".EVA") || path_str.ends_with(".EVN") {
            plate_appearances.append(&mut read_plate_appearances_from_file(
                &player_id,
                player_name,
                &path,
            ));
        }
    }
    return plate_appearances;
}

fn read_plate_discipline_from_file(player_name: &String, year: i32) -> (f32, f32, f32) {
    let path_text = format!("{DATA_DIR}/{}eve/{}_plate_discipline.csv", year, year);
    let path_in = Path::new(&path_text);
    let contents = fs::read_to_string(path_in).expect("Something went wrong reading plate discipline file");
    // Split data by newline
    let lines = contents.split("\n");
    println!("{}", player_name);
    for line in lines {
        // Remove trailing newline character
        let mut line_trimmed = line.to_string();
        line_trimmed.pop();
        // Check if the line contains the player's name
        let line_data = line_trimmed.split("\t").collect::<Vec<&str>>();
        if line_data.len() < 10 {
            continue;
        }
        if line_data[1] == player_name {
            let mut oswing_pct_string = line_data[3].to_string();
            oswing_pct_string.pop(); // removed percent sign
            let oswing_pct = match oswing_pct_string.parse::<f32>() {
                Ok(oswing_pct) => oswing_pct,
                Err(e) => panic!(
                    "Error parsing oswing_pct string: {}, ({})",
                    oswing_pct_string, e
                ),
            };
            let mut swing_pct_string = line_data[5].to_string();
            swing_pct_string.pop(); // removed percent sign
            let swing_pct = match swing_pct_string.parse::<f32>() {
                Ok(swing_pct) => swing_pct,
                Err(e) => panic!(
                    "Error parsing oswing_pct string: {}, ({})",
                    swing_pct_string, e
                ),
            };
            let mut zone_pct_string = line_data[9].to_string();
            zone_pct_string.pop(); // removed newline
            zone_pct_string.pop(); // removed percent sign
            let zone_pct = match zone_pct_string.parse::<f32>() {
                Ok(zone_pct) => zone_pct,
                Err(e) => panic!(
                    "Error parsing zone_pct string: {}, ({})",
                    zone_pct_string, e
                ),
            };
            return (oswing_pct, swing_pct, zone_pct);
        }
    }
    (-1.0, -1.0, -1.0)
}

fn parse_input_arguments(args: Vec<String>) -> (String, String, i32) {
    if args.len() < 4 {
        if args[1] == "all" {
            let player_name = args[1].to_owned();
            let team_name = "".to_owned();
            let year = args[2].parse::<i32>().unwrap();
            return (player_name, team_name, year);
        }
        panic!("Please provide a player name, a team name, and a year as arguments");
    }
    let player_name = args[1].to_owned();
    let team_name = args[2].to_owned();
    let year = args[3].parse::<i32>().unwrap();
    return (player_name, team_name, year);
}

fn read_all_player_names(year: i32) -> Vec<(String, String)> {
    let mut player_team_names: Vec<(String, String)> = Vec::new();
    for dir in std::fs::read_dir(format!("{DATA_DIR}/{}eve/", year)).unwrap() {
        let path = dir.unwrap().path();
        let path_str = path.to_str().unwrap();
        if path_str.ends_with(".ROS") {
            let team_name = path_str[(path_str.len()-11)..(path_str.len()-8)].to_owned();
            let contents_roster =
                fs::read_to_string(path).expect("Something went wrong reading roster files");
            let roster_lines = contents_roster.split("\n").collect::<Vec<&str>>();
            for roster_line in roster_lines {
                let line_data = roster_line.split(",").collect::<Vec<&str>>();
                if line_data.len() > 2 {
                    let mut player_name = line_data[2].to_owned();
                    player_name.push_str(" ");
                    player_name.push_str(line_data[1]);
                    player_team_names.push((player_name, team_name.clone()));
                }
            }
        }
    }
    return player_team_names;
}

fn sim_player_with_and_without_bat(player_name: &String, team_name: &String, year: i32) -> (f32, f32) {
    let (oswing_pct, swing_pct, zone_pct) = read_plate_discipline_from_file(player_name, year);
    if oswing_pct == -1.0 {
        // probably a pitcher
        return (0.0, 0.0);
    }
    let plate_appearances = read_in_plate_appearances(player_name, team_name, year);
    let mut plate_appearances_no_bat: Vec<PlateAppearance> = Vec::new();
    for appearance in &plate_appearances {
        plate_appearances_no_bat.push(simulate_plate_appearance_no_bat(
            appearance, oswing_pct, swing_pct, zone_pct,
        ));
    }
    let obp = calculate_obp(&plate_appearances);
    let obp_no_bat = calculate_obp(&plate_appearances_no_bat);
    return (obp, obp_no_bat);
}

fn main() {
    // Collect input arguments
    let args: Vec<String> = std::env::args().collect();
    let (player_name, team_name, year) = parse_input_arguments(args);

    // If player_name argument is "all", then return a top 20 list of players with the highest OBP without a bat
    if player_name == "all" {
        let player_team_names = read_all_player_names(year);
        let mut obp_no_bat_list: Vec<(String, f32, f32)> = Vec::new();
        for (player_name, team_name) in player_team_names {
            let (obp, obp_no_bat) = sim_player_with_and_without_bat(&player_name, &team_name, year);
            obp_no_bat_list.push((player_name, obp, obp_no_bat));
        }
        obp_no_bat_list.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        println!("Top 20 OBP without a bat in {}", year);
        for (i, (player_name, obp, obp_no_bat)) in obp_no_bat_list.iter().enumerate() {
            if i >= 20 {
                break;
            }
            println!("{}: {}, {}", player_name, obp, obp_no_bat);
        }
        return;
    } else {
         // Calculate the player's OBP for 2023 with and without bat
        let (obp, obp_no_bat) = sim_player_with_and_without_bat(&player_name, &team_name, year);
        println!(
            "OBP for {} in {}: {}",
            player_name,
            year,
            obp
        );
        println!(
            "OBP for {} in {} without a bat: {}",
            player_name,
            year,
            obp_no_bat
        );
    }
}
