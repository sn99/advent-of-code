use std::collections::HashMap;
fn part_1(input: &str) {
    let colors: HashMap<&'static str, u32> = [("red", 12u32), ("green", 13u32), ("blue", 14u32)]
        .iter()
        .cloned()
        .collect();

    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let game = line.split(": ").collect::<Vec<&str>>();
            let mut game_id: u32 = game[0].split("Game ").collect::<String>().parse().unwrap();
            let sets = game[1].split("; ");

            for set in sets {
                let items = set.split(", ");
                for item in items {
                    let cube = item.split(' ').collect::<Vec<&str>>();
                    let color = cube[1];
                    let count: u32 = cube[0].parse().unwrap();
                    game_id = match colors.get(color) {
                        Some(&color_value) if count > color_value => 0,
                        _ => game_id,
                    };
                }
            }

            Some(game_id)
        })
        .sum();
    println!("{sum}");
}

fn part_2(input: &str) {
    let sum: u32 = input
        .lines()
        .filter_map(|line| {
            let mut counter = HashMap::with_capacity(3);
            let game = line.split(": ").collect::<Vec<&str>>();
            let sets = game[1].split("; ");

            for set in sets {
                let items = set.split(", ");
                for item in items {
                    let cube = item.split(' ').collect::<Vec<&str>>();
                    let color = cube[1];
                    let count: u32 = cube[0].parse().unwrap();

                    let prev_count = counter.entry(color).or_insert(1);
                    *prev_count = (*prev_count).max(count);
                }
            }

            Some(counter.values().product::<u32>())
        })
        .sum();

    println!("{sum}");
}

fn main() {
    let input = std::fs::read_to_string("resources/day_2").expect("Unable to read lines");
    part_1(&input);
    part_2(&input);
}
