use regex::Regex;

pub fn exec(lines:Vec<&str>) {
    let digit_re = Regex::new(r"^(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut sum = 0;
    for line in lines {
        let matches = (0..(line.len()))
            .map(|offset| (offset, digit_re.find(&line[offset..])))
            .filter_map(|(offset, maybe_match)| maybe_match.map(|m| (offset, m)))
            .collect::<Vec<_>>();
        let mut words = vec![matches.first().unwrap(), matches.last().unwrap()]
            .into_iter()
            .map(|(offset, m)| &line[m.start() + offset..m.end() + offset]);
        sum += 10 * word_to_digit(words.next().unwrap()) + word_to_digit(words.next().unwrap());
    }
   //  println!("{sum}");
}

fn word_to_digit(word: &str) -> usize {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => str::parse::<usize>(word).unwrap(),
    }
}


fn main() {
    let lines:Vec<_> =include_str!("../../input.txt").lines().collect();
    exec(lines);
}