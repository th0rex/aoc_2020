use std::collections::HashSet;

fn main() {
    let s = include_str!("inputs/day_1_1.txt");

    let mut set = HashSet::new();

    for l in s.lines() {
        let i = l.parse::<usize>().unwrap();
        set.insert(2020 - i);

        if set.contains(&i) {
            let other = 2020 - i;
            println!("{}", other * i);
            break;
        }
    }
}
