const INPUT: &str = "0,6,1,7,2,19,20";

fn main() {
    const END: usize = 30000001; //2021 for Part 1
    let calc_num = |spoken: Option<usize>, curr_turn: usize| -> (usize, usize) {
        (
            match spoken {
                Some(prev_turn) => curr_turn - prev_turn - 1,
                None => 0,
            },
            curr_turn,
        )
    };
    // let mut record: HashMap<usize, usize> = HashMap::new(); // let mut speak = |(n, i)| -> (Option<usize>, usize) { (record.insert(n, i), n) };
    let mut record = vec![None; END]; // can't use array becuase of stack overflow. could use the nightly box syntax, but meh
    let mut speak =
        |(n, i)| -> (Option<usize>, usize) { (std::mem::replace(&mut record[n], Some(i)), n) };

    let ((spoken_before, last_num), cursor) = INPUT
        .split(',')
        .map(|s| s.parse::<usize>().expect("parsing err"))
        .fold(((None, 0), 1), |((_, _), cursor), number| {
            (speak((number, cursor)), cursor + 1)
        });

    let (_, last_num) = (cursor..END)
        .fold((spoken_before, last_num), |(spoken_before, _), cursor| {
            speak(calc_num(spoken_before, cursor))
        });

    println!("Day 15 Part 1&2 (just change END): {}", last_num);
}
