/// Returns the original string with 'one' replaced with '1', 'two' replaced with '2' etc.
fn convert_word_to_number(word: &str) -> String {
    // replace 'one' with '1', 'two' with '2' etc.
    let word = word.to_owned();
    let word = word.replace("one", "1");
    let word = word.replace("two", "2");
    let word = word.replace("three", "3");
    let word = word.replace("four", "4");
    let word = word.replace("five", "5");
    let word = word.replace("six", "6");
    let word = word.replace("seven", "7");
    let word = word.replace("eight", "8");
    let word = word.replace("nine", "9");

    word
}

fn sum_combined_first_and_last_digit(lines: &Vec<&str>) -> u32 {
    let mut sum = 0;
    for line in lines {
        // Get all numbers
        let numbers = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();

        // Get first and last number
        let first = numbers[0];
        let last = numbers[numbers.len() - 1];

        // Combine into 2-digit number
        let num = format!("{}{}", first, last).parse::<u32>().unwrap();
        println!("{} + {} = {}, word: {}", first, last, num, line);
        // Add to sum
        sum += num;
    }

    sum
}

fn main() {
    // Open input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Split input into lines
    let lines = input.lines();
    let lines_as_numbers = lines
        .clone()
        .map(|line| convert_word_to_number(line))
        .collect::<Vec<String>>();

    let puzzle1_sum: u32 = sum_combined_first_and_last_digit(&lines.clone().collect::<Vec<_>>());
    let puzzle2_sum = sum_combined_first_and_last_digit(
        &lines_as_numbers
            .iter()
            .map(|line| line.as_str())
            .collect::<Vec<_>>(),
    );

    // Print sum
    println!("puzzle1_sum: {}", puzzle1_sum);
    println!("puzzle2_sum: {}", puzzle2_sum);
}
