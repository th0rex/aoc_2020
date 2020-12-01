use std::collections::HashSet;

fn main() {
    let s = include_str!("inputs/day_1_1.txt");
    let nums = s.lines().map(|x| x.parse().unwrap()).collect::<Vec<usize>>();

    let mut set = HashSet::new();

    'outer: for n in &nums {
        set.clear();
        let target = 2020 - n;

        for &k in &nums {
            if target < k { continue; }
            set.insert(target - k);

            if set.contains(&k) {
                let other = 2020 - n - k;
                println!("{}", n * k * other);
                break 'outer;
            }
        }
    }
}
