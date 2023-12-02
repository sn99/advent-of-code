// including first and last characters of original str in replacement because some values in input have overlapping start/end characters
const REPLACEMENT_PAIRS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

fn part_1(input: String) {
    //let input = std::fs::read_to_string("resources/day_1").expect("Unable to read lines");

    let lines = input.lines();
    let mut sum = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let (mut front_index, mut back_index) = (0, chars.len() - 1);
        let mut number = 0;

        while front_index <= back_index {
            match (
                chars[front_index].is_numeric(),
                chars[back_index].is_numeric(),
            ) {
                (true, true) => {
                    number = (chars[front_index]
                        .to_digit(10)
                        .expect("Failed to convert digit")
                        * 10)
                        + chars[back_index]
                            .to_digit(10)
                            .expect("Failed to convert digit");
                    break;
                }
                (false, true) => {
                    front_index += 1;
                }
                (true, false) => {
                    back_index -= 1;
                }
                (false, false) => {
                    front_index += 1;
                    back_index -= 1;
                }
            }
        }

        sum += number;
    }

    println!("{}", sum);
}

fn part_2(mut input: String) {
    //let mut input = std::fs::read_to_string("resources/day_1").expect("Unable to read lines");

    for pair in REPLACEMENT_PAIRS {
        input = input.replace(pair.0, pair.1);
    }

    part_1(input)
}
fn main() {
    let input = std::fs::read_to_string("resources/day_1").expect("Unable to read lines");
    part_1(input.clone());
    part_2(input);
}
