use anyhow::{Context, Result};
use log::trace;
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use std::collections::HashMap;
use std::env;

use crate::structs::{APIResponse, Calendar, Cli};

pub fn post(args: &Cli) -> Result<Calendar, anyhow::Error> {
    let mut map = HashMap::new();
    map.insert(
        "query",
        format!(
            "{{
                user(login: \"{user}\") {{
                    contributionsCollection {{
                    contributionCalendar {{
                        totalContributions
                        weeks {{
                        contributionDays {{
                            contributionCount
                            color
                            date
                            weekday
                        }}
                        firstDay
                        }}
                    }}
                    }}
                }}
            }}",
            user = &args.user
        ),
    );

    let client = reqwest::blocking::Client::new();

    trace!("Loading GITHUB_TOKEN environment variable...");
    let github_token = env::var("GITHUB_TOKEN")
        .with_context(|| "Unable to find GITHUB_TOKEN environment variable")?;
    trace!("Successfully loaded GITHUB_TOKEN environment variable.");

    trace!("Sending GraphQL POST request...");
    let json = client
        .post("https://api.github.com/graphql")
        .header(USER_AGENT, "Contribution collector")
        .header(AUTHORIZATION, format!("bearer {}", github_token))
        .json(&map)
        .send()?
        .json::<APIResponse>()?;
    trace!("Successfully sent GraphQL POST request.");

    // Convert from APIResponse down to Calendar
    let calendar = json.data.user.contributions.calendar;

    Ok(calendar)
}
