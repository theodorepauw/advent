mod twenty17;
mod twenty18;
mod twenty19;
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

    let start = std::time::Instant::now();
    let solution_handles: Vec<std::thread::JoinHandle<util::Result<()>>> = args
        .filter_map(|a| a.parse::<u8>().ok())
        .map(|day| {
            std::thread::spawn(move || match (year, &day) {
                (2017, 1) => twenty17::day01::solve(),
                (2017, 2) => twenty17::day02::solve(),
                (2017, 3) => twenty17::day03::solve(),
                (2017, 4) => twenty17::day04::solve(),
                (2017, 5) => twenty17::day05::solve(),
                (2017, 6) => twenty17::day06::solve(),
                (2017, 7) => twenty17::day07::solve(),
                (2017, 8) => twenty17::day08::solve(),
                (2017, 9) => twenty17::day09::solve(),
                (2017, 10) => twenty17::day10::solve(),
                (2017, 11) => twenty17::day11::solve(),
                (2017, 12) => twenty17::day12::solve(),
                (2017, 13) => twenty17::day13::solve(),
                (2017, 14) => twenty17::day14::solve(),
                (2017, 15) => twenty17::day15::solve(),
                (2017, 16) => twenty17::day16::solve(),
                (2017, 17) => twenty17::day17::solve(),
                (2017, 18) => twenty17::day18::solve(),
                (2017, 19) => twenty17::day19::solve(),
                (2017, 20) => twenty17::day20::solve(),
                (2017, 21) => twenty17::day21::solve(),
                (2017, 22) => twenty17::day22::solve(),
                (2017, 23) => twenty17::day23::solve(),
                (2017, 24) => twenty17::day24::solve(),
                (2017, 25) => twenty17::day25::solve(),
                (2018, 1) => twenty18::day01::solve(),
                (2018, 2) => twenty18::day02::solve(),
                (2018, 3) => twenty18::day03::solve(),
                (2018, 4) => twenty18::day04::solve(),
                (2018, 5) => twenty18::day05::solve(),
                (2018, 6) => twenty18::day06::solve(),
                // (2018, 7) => twenty18::day07::solve(),
                // (2018, 8) => twenty18::day08::solve(),
                // (2018, 9) => twenty18::day09::solve(),
                // (2018, 10) => twenty18::day10::solve(),
                // (2018, 11) => twenty18::day11::solve(),
                // (2018, 12) => twenty18::day12::solve(),
                // (2018, 13) => twenty18::day13::solve(),
                // (2018, 14) => twenty18::day14::solve(),
                // (2018, 15) => twenty18::day15::solve(),
                // (2018, 16) => twenty18::day16::solve(),
                // (2018, 17) => twenty18::day17::solve(),
                // (2018, 18) => twenty18::day18::solve(),
                // (2018, 19) => twenty18::day19::solve(),
                // (2018, 20) => twenty18::day20::solve(),
                // (2018, 21) => twenty18::day21::solve(),
                // (2018, 22) => twenty18::day22::solve(),
                // (2018, 23) => twenty18::day23::solve(),
                // (2018, 24) => twenty18::day24::solve(),
                // (2018, 25) => twenty18::day25::solve(),
                (2019, 1) => twenty19::day01::solve(),
                (2019, 2) => twenty19::day02::solve(),
                (2019, 3) => twenty19::day03::solve(),
                (2019, 4) => twenty19::day04::solve(),
                // (2019, 5) => twenty19::day05::solve(),
                // (2019, 6) => twenty19::day06::solve(),
                // (2019, 7) => twenty19::day07::solve(),
                // (2019, 8) => twenty19::day08::solve(),
                // (2019, 9) => twenty19::day09::solve(),
                // (2019, 10) => twenty19::day10::solve(),
                // (2019, 11) => twenty19::day11::solve(),
                // (2019, 12) => twenty19::day12::solve(),
                // (2019, 13) => twenty19::day13::solve(),
                // (2019, 14) => twenty19::day14::solve(),
                // (2019, 15) => twenty19::day15::solve(),
                // (2019, 16) => twenty19::day16::solve(),
                // (2019, 17) => twenty19::day17::solve(),
                // (2019, 18) => twenty19::day18::solve(),
                // (2019, 19) => twenty19::day19::solve(),
                // (2019, 20) => twenty19::day20::solve(),
                // (2019, 21) => twenty19::day21::solve(),
                // (2019, 22) => twenty19::day22::solve(),
                // (2019, 23) => twenty19::day23::solve(),
                // (2019, 24) => twenty19::day24::solve(),
                // (2019, 25) => twenty19::day25::solve(),
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
                (2020, 21) => twenty20::day21::solve(),
                (2020, 22) => twenty20::day22::solve(),
                (2020, 23) => twenty20::day23::solve(),
                (2020, 24) => twenty20::day24::solve(),
                (2020, 25) => twenty20::day25::solve(),
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

    writeln!(
        io::stdout(),
        "Finished in {} ms",
        start.elapsed().as_millis()
    )?;
    Ok(())
}
