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
        let n = rest.chars().filter(|&x| x == c).count();

        if n >= low && n <= high {
            num_valid += 1;
        }
    }

    println!("{}", num_valid);
}
