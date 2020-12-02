fn parse_line(mut line: &str) -> ((usize, usize), char, &str) {
    let idx = line.find('-').unwrap();
    let low = &line[..idx];

    line = &line[idx + 1..];
    let idx = line.find(' ').unwrap();
    let high = &line[..idx];

    line = &line[idx + 1..];
    let c = line.chars().next().unwrap();

    let rest = &line[3..];

    ((low.parse().unwrap(), high.parse().unwrap()), c, rest)
}

fn main() {
    let lines = include_str!("inputs/day_2_1.txt").lines().collect::<Vec<_>>();

    let mut num_valid = 0;

    for l in &lines {
        let ((low, high), c, rest) = parse_line(l);
        let rest = rest.as_bytes();
        let c = c as u8;
        let first = rest.get(low - 1);
        let second = rest.get(high - 1);
        match (first, second) {
            (Some(&a), Some(&b)) if a == c && b != c => num_valid += 1,
            (Some(&a), Some(&b)) if a != c && b == c => num_valid += 1,
            (Some(&a), None)     if a == c           => num_valid += 1,
            (None,     Some(&b)) if b == c           => num_valid += 1,
            _ => {},
        }
    }

    println!("{}", num_valid);
}
