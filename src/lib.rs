use core::panic;
use rand::Rng;
use std::fmt;

#[derive(Clone)]
pub struct Date {
    year: i32,
    month: i32,
    day: i32,
}

impl Date {
    pub fn new(year: i32, month: i32, day: i32) -> Self {
        Self { year, month, day }
    }
    pub fn clone(&self) -> Self {
        Self {
            year: self.year,
            month: self.month,
            day: self.day,
        }
    }
    pub fn get_year(&self) -> &i32 {
        &self.year
    }
    pub fn get_month(&self) -> &i32 {
        &self.month
    }
    pub fn get_day(&self) -> &i32 {
        &self.day
    }
}

pub fn convert_string_to_date(date_string: &str) -> Date {
    let date_data = date_string.split("/").collect::<Vec<&str>>();
    let year = match date_data[0].parse::<i32>() {
        Ok(year) => year,
        Err(e) => panic!("Error parsing year string: {}, ({})", date_data[0], e),
    };
    let month = match date_data[1].parse::<i32>() {
        Ok(month) => month,
        Err(e) => panic!("Error parsing month string: {}, ({})", date_data[1], e),
    };
    let day = match date_data[2].parse::<i32>() {
        Ok(day) => day,
        Err(e) => panic!("Error parsing day string: {}, ({})", date_data[2], e),
    };
    Date { year, month, day }
}

#[derive(Clone)]
pub struct PlateAppearance {
    date: Date,
    batter: String,
    outcome: char,
    pitches: Vec<char>,
}

impl PlateAppearance {
    pub fn new(date: Date, batter: String, outcome: char, pitches: Vec<char>) -> Self {
        Self {
            date,
            batter,
            outcome,
            pitches,
        }
    }
    pub fn get_date(&self) -> &Date {
        &self.date
    }
    pub fn get_batter(&self) -> &String {
        &self.batter
    }
    pub fn get_outcome(&self) -> &char {
        &self.outcome
    }
    pub fn get_pitches(&self) -> &Vec<char> {
        &self.pitches
    }
}

impl fmt::Display for PlateAppearance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Date: {}/{}/{}, Pitches: {:?}, Outcome: {}",
            self.date.get_year(),
            self.date.get_month(),
            self.date.get_day(),
            self.pitches,
            self.outcome
        )
    }
}

// Create some integer mappings for the different pitch results
pub fn simplify_pitch_codes(pitch: char) -> char {
    match pitch {
        'A' => 'C', // automatic strike
        'B' => 'B', // ball
        'C' => 'C', // called strike
        'F' => 'F', // foul ball, need re-sim without bat
        'H' => 'H', // hit by pitch
        'I' => 'I', // intentional ball
        'L' => 'F', // foul bunt, need re-sim without bat
        'M' => 'S', // missed bunt attempt, need re-sim without bat
        'N' => 'N', // no pitch (on balks and interference calls), not sure what to do
        'O' => 'F', // foul tip on bunt, need re-sim without bat
        'P' => 'B', // pitchout
        'Q' => 'C', // swinging on pitchout
        'R' => 'F', // foul ball on pitchout, need re-sim without bat
        'S' => 'S', // swinging strike, need re-sim without bat
        'T' => 'F', // foul tip, need re-sim without bat
        'U' => 'B', // unknown or missed pitch
        'V' => 'B', // called ball because pitcher went to his mouth
        'X' => 'X', // ball put into play, need re-sim without bat
        'Y' => 'X', // ball put into play on pitchout, need re-sim without bat
        _ => 'N',   // unknown
    }
}

pub fn simplify_outcome_codes(outcome: char) -> char {
    match outcome {
        // Look these up here https://www.retrosheet.org/eventfile.htm#5
        'S' => 'S', // single
        'D' => 'D', // double
        'T' => 'T', // triple
        'H' => 'H', // home run
        'W' => 'W', // walk
        'I' => 'I', // intentional walk
        'E' => 'E', // error, don't include in OBP
        'F' => 'O', // foul out
        'K' => 'K', // strikeout
        '0' => 'O', // fielded out
        '1' => 'O', // fielded out
        '2' => 'O', // fielded out
        '3' => 'O', // fielded out
        '4' => 'O', // fielded out
        '5' => 'O', // fielded out
        '6' => 'O', // fielded out
        '7' => 'O', // fielded out
        '8' => 'O', // fielded out
        '9' => 'O', // fielded out
        _ => 'N',   // unknown, ignore
    }
}

pub fn calculate_obp(plate_appearances: &Vec<PlateAppearance>) -> f32 {
    let mut hits = 0;
    let mut walks = 0;
    let mut at_bats = plate_appearances.len();
    for plate_appearance in plate_appearances {
        let outcome = plate_appearance.get_outcome();
        if outcome == &'S' || outcome == &'D' || outcome == &'T' || outcome == &'H' {
            hits += 1;
        }
        if outcome == &'W' || outcome == &'I' {
            walks += 1;
        }
        if outcome == &'E' {
            at_bats -= 1;
        }
    }
    return (hits + walks) as f32 / at_bats as f32;
}

fn simulate_until_outcome(zone_pct: f32, mut balls: i32, mut strikes: i32) -> (char, Vec<char>) {
    let mut simulated_pitches: Vec<char> = Vec::new();
    while balls < 4 && strikes < 3 {
        let zone = rand::thread_rng().gen_range(0.0..1.0);
        if zone <= zone_pct {
            simulated_pitches.push('C');
            strikes += 1;
        } else {
            simulated_pitches.push('B');
            balls += 1;
        }
    }
    if balls == 4 {
        return ('W', simulated_pitches);
    } else {
        return ('K', simulated_pitches);
    }
}

pub fn simulate_plate_appearance_no_bat(
    appearance: &PlateAppearance,
    oswing_pct: f32,
    zone_pct: f32,
) -> PlateAppearance {
    let mut balls = 0;
    let mut strikes = 0;
    let mut pitches_no_bat: Vec<char> = Vec::new();
    for pitch in &appearance.pitches {
        // B, balls stay balls,
        // C, strikes with no swing stay strikes,
        // I, intentional walks stay intentional walks
        // H, hit by pitches stay hit by pitches
        // S, strikes with swing require us to re-simulate the pitch without the bat
        // F, foul balls require us to re-simulate the pitch without the bat
        // X, ball put into play requires us to re-simulate the pitch
        // if we still have no outcome by the end, we need to simulate until we get one
        let pitch = pitch.to_owned();
        if pitch == 'B' || pitch == 'C' || pitch == 'I' || pitch == 'H' {
            pitches_no_bat.push(pitch);
            if pitch == 'B' || pitch == 'I' {
                balls += 1;
            } else if pitch == 'C' {
                strikes += 1;
            } else if pitch == 'H' {
                balls = 4;
            }
        } else if pitch == 'S' || pitch == 'F' || pitch == 'X' {
            // Batter swung at the pitch, so we need to re-simulate the pitch without the bat
            // Probability that it was inside the strike zone
            let zone = rand::thread_rng().gen_range(0.0..1.0);
            if zone <= oswing_pct {
                pitches_no_bat.push('B');
                balls += 1;
            } else {
                pitches_no_bat.push('C');
                strikes += 1;
            }
        }
    }

    let outcome_no_bat;
    if balls == 4 {
        outcome_no_bat = 'W';
    } else if strikes == 3 {
        outcome_no_bat = 'K';
    } else {
        let (simulated_outcome, simulated_pitches) =
            simulate_until_outcome(zone_pct, balls, strikes);
        outcome_no_bat = simulated_outcome;
        for pitch in simulated_pitches {
            pitches_no_bat.push(pitch);
        }
    }
    let simulated_appearance = PlateAppearance::new(
        appearance.date.clone(),
        appearance.batter.clone(),
        outcome_no_bat,
        pitches_no_bat,
    );
    return simulated_appearance;
}
