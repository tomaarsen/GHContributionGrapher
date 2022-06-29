use crate::structs::{Calendar, Cli, HexToRGB};
use log::trace;

fn print_text_colored(text: &str, color: &str) -> Result<(), std::io::Error> {
    // Given a string with a hex color and a text, print the text in that color.
    color
        .to_rgb()
        .paint(text.as_bytes())
        .write_to(&mut std::io::stdout())
}

fn print_total_contributions(calendar: &Calendar, args: &Cli) {
    trace!("Printing the total contributions header...");
    let bar_length = calendar.total_contributions.to_string().len() + args.user.len() + 38;
    // Print the line above the contributions
    println!("╔{}╗", "═".repeat(bar_length));
    // Print the body of the contributions
    println!(
        "║ {} contributions in the last year by {}. ║",
        calendar.total_contributions, args.user
    );
    // Print the line above the header, below the contributions
    println!(
        "╠{}╩{}╗",
        "═".repeat(bar_length),
        "═".repeat(109 - bar_length)
    );
    trace!("Printed the total contributions header...");
}

fn print_calendar_header(calendar: &Calendar) -> Result<(), anyhow::Error> {
    trace!("Printing the calendar month header...");
    let months = vec![
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    // This backlog tracks how many spaces should *not* be printed
    let mut backlog = 0;

    // Left bar
    print!("║    ");
    for week in &calendar.weeks {
        for day in &week.days {
            // Only if it's the first of some month, look for the month and print it.
            // Increment the backlog by 3 to prevent 3 spaces from being printed.
            if day.to_day_i()? == 1 {
                let month_id = day.to_month_i()? - 1;
                print!("{}", months[month_id as usize]);
                backlog += 3;
            }
        }
        // Either absorb the backlog, or print 2 spaces (or both partially).
        for _ in 0..2 {
            if backlog > 0 {
                backlog -= 1;
            } else {
                print!(" ");
            }
        }
    }
    // Right bar
    println!("║");
    // Print the line below the header
    println!("{}{}{}", "╠", "═".repeat(110), "╣");

    trace!("Printed the calendar month header.");
    Ok(())
}

fn print_calendar_body(calendar: &Calendar) -> Result<(), anyhow::Error> {
    trace!("Printing the calendar body...");
    let weekday_strings = vec!["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
    for (weekday_i, weekday) in weekday_strings.iter().enumerate() {
        // Print the weekday information on the left of every row
        print!("║ {} ", weekday);

        // Print each row
        for week in &calendar.weeks {
            if &weekday_i < &week.days.len() {
                let day = &week.days[weekday_i];
                // If the contribution is 0 for the day, then print emptiness
                // Alternatively, grab the color, convert it to Colour, paint it,
                // and then print it out
                let char = match (day.to_day_i()?, day.count) {
                    (1, _) => "x ",
                    (_, 0) => "  ",
                    (_, _) => "■ ",
                };
                let color = match day.color.as_str() {
                    "#9be9a8" => "#382F29",
                    "#40c463" => "#FCA311",
                    "#30a14e" => "#FC6A03",
                    "#216e39" => "#D91B1B",
                    _ => "#000000",
                };
                print_text_colored(char, color)?;
            } else {
                // Otherwise, print two spaces to make the body rectangular
                print!("  ");
            }
        }
        // Wrap up the row with a newline and a bar
        println!("║");
    }
    trace!("Printed the calendar body...");

    Ok(())
}

fn print_calendar_footer() -> Result<(), anyhow::Error> {
    trace!("Printing the calendar footer...");
    // The 4 non-zero colors
    let colors = ["#382F29", "#FCA311", "#FC6A03", "#D91B1B"];
    // Print the left side, including padding
    print!("║ x denotes the start of each month{}", " ".repeat(55));
    print!(" Less   ");
    for color in colors {
        print_text_colored("■ ", color)?;
    }
    println!("More ║");
    // Print the bottom bar
    println!("{}{}{}", "╚", "═".repeat(110), "╝");
    trace!("Printed the calendar footer.");

    Ok(())
}

pub fn print_calendar(calendar: &Calendar, args: &Cli) -> Result<(), anyhow::Error> {
    // Print the entire GitHub contribution calendar, starting from
    // the number of contributions, then the header, the body and the footer.
    print_total_contributions(&calendar, args);
    print_calendar_header(&calendar)?;
    print_calendar_body(&calendar)?;
    print_calendar_footer()?;
    Ok(())
}
