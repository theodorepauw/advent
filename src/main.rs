mod twenty20;
mod util;
use std::io::{self, Write};

fn main() -> util::Result<()> {
    let mut args = std::env::args();
    let year = args
        .nth(1)
        .ok_or("Specify a year!")?
        .parse::<u16>()
        .map(|y| {
            if y > 2014 && y < 2021 {
                Ok(y)
            } else {
                Err("Year must be a number 2015-2020")
            }
        })??;

    let solution_handles: Vec<std::thread::JoinHandle<util::Result<()>>> = args
        .filter_map(|a| a.parse::<u8>().ok())
        .map(|day| {
            std::thread::spawn(move || match (year, &day) {
                (2020, 1) => twenty20::day01::solve(),
                (2020, 2) => twenty20::day02::solve(),
                (2020, 3) => twenty20::day03::solve(),
                (2020, 4) => twenty20::day04::solve(),
                (2020, 5) => twenty20::day05::solve(),
                (2020, 6) => twenty20::day06::solve(),
                (2020, 7) => twenty20::day07::solve(),
                (2020, 8) => twenty20::day08::solve(),
                (2020, 9) => twenty20::day09::solve(),
                (2020, 10) => twenty20::day10::solve(),
                (2020, 11) => twenty20::day11::solve(),
                (2020, 12) => twenty20::day12::solve(),
                (2020, 13) => twenty20::day13::solve(),
                (2020, 14) => twenty20::day14::solve(),
                (2020, 15) => twenty20::day15::solve(),
                (2020, 16) => twenty20::day16::solve(),
                (2020, 17) => twenty20::day17::solve(),
                (2020, 18) => twenty20::day18::solve(),
                (2020, 19) => twenty20::day19::solve(),
                (2020, 20) => twenty20::day20::solve(),
                _ => {
                    writeln!(
                        io::stdout(),
                        "Year {} Day {} has not been implemented yet",
                        year,
                        day
                    )?;
                    Ok(())
                }
            })
        })
        .collect();

    for h in solution_handles {
        if let Err(e) = h.join() {
            writeln!(io::stdout(), "{:?}", e)?;
        }
    }
    Ok(())
}
