fn main() {
    let s = include_str!("inputs/day_5.txt");
    let mut nums = s
        .lines()
        .map(|x| {
            x.chars().fold(0, |acc, x| {
                (acc << 1)
                    | match x {
                        'R' | 'B' => 1,
                        'L' | 'F' => 0,
                        _ => unreachable!(),
                    }
            })
        })
        .collect::<Vec<usize>>();
    println!("{}", nums.iter().max().unwrap());

    nums.sort();
    println!(
        "{}",
        nums.windows(2).find(|x| x[1] - x[0] == 2).unwrap()[0] + 1
    );
}
