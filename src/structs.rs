use ansi_term::Colour;
use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    // The GitHub user to crawl for
    #[clap()]
    pub user: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(alias = "contributionsCollection")]
    pub contributions: Contributions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contributions {
    #[serde(alias = "contributionCalendar")]
    pub calendar: Calendar,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Calendar {
    #[serde(alias = "totalContributions")]
    pub total_contributions: i32,
    pub weeks: Vec<Week>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Week {
    #[serde(alias = "contributionDays")]
    pub days: Vec<Day>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    #[serde(alias = "contributionCount")]
    pub count: i32,
    pub color: String,
    pub date: String,
    pub weekday: usize,
}

impl Day {
    fn to_i(&self, i: usize) -> Result<i32, std::num::ParseIntError> {
        self.date.split("-").nth(i).unwrap().parse::<i32>()
    }

    pub fn to_month_i(&self) -> Result<i32, std::num::ParseIntError> {
        self.to_i(1)
    }

    pub fn to_day_i(&self) -> Result<i32, std::num::ParseIntError> {
        self.to_i(2)
    }
}

pub trait HexToRGB {
    fn to_rgb(&self) -> Colour;
}

impl HexToRGB for str {
    // Convert a hex color string to a Colour instance
    fn to_rgb(&self) -> Colour {
        let v = i64::from_str_radix(&self[1..], 16).unwrap() as f64;
        let r: u8 = (v / 256_f64.powf(2.0) % 256.0) as u8;
        let g: u8 = (v / 256_f64.powf(1.0) % 256.0) as u8;
        let b: u8 = (v / 256_f64.powf(0.0) % 256.0) as u8;
        Colour::RGB(r, g, b)
    }
}
