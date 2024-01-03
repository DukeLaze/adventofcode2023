/// Returns the original string with 'one' replaced with '1', 'two' replaced with '2' etc.
fn convert_word_to_number(input: &str, replace_word_digits: bool) -> i8 {
    let word = input.to_owned();
    let mut numbers = Vec::new();

        for i in 0..word.len() {
            let char = word.chars().nth(i).unwrap();
            if char.is_digit(10) {
                let num = char.to_digit(10).unwrap() as i8;
                numbers.push(num);
            }
            else if replace_word_digits {
                // Check if word[i..] starts with 'one', 'two' etc.
                if word[i..].starts_with("one") { numbers.push(1) }
                else if word[i..].starts_with("two") { numbers.push(2) }
                else if word[i..].starts_with("three") { numbers.push(3) }
                else if word[i..].starts_with("four") { numbers.push(4) }
                else if word[i..].starts_with("five") { numbers.push(5) }
                else if word[i..].starts_with("six") { numbers.push(6) }
                else if word[i..].starts_with("seven") { numbers.push(7) }
                else if word[i..].starts_with("eight") { numbers.push(8) }
                else if word[i..].starts_with("nine") { numbers.push(9) }
            }
        }
    

    // Get first and last number
    let first = numbers[0];
    let last = numbers[numbers.len() - 1];

    let num = format!("{}{}", first, last).parse::<i8>().unwrap();

    assert!(num >= 0 && num <= 99);
    num
}

fn sum_combined_first_and_last_digit(lines: &Vec<&str>, replace_word_digits: bool) -> u32 {
    let mut sum: u32 = 0;
    for line in lines {
        let num = convert_word_to_number(&line, replace_word_digits);
        // Add to sum
        sum += num as u32;
    }

    sum
}

fn main() {
    // Open input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Convert to lowercase
    let input = input.to_lowercase();

    // Split input into lines
    let lines: Vec<&str> = input.lines().collect();
    let puzzle2_sum = sum_combined_first_and_last_digit(&lines, true);

    let puzzle1_sum = sum_combined_first_and_last_digit(&lines, false);

    // Print sum
    println!("puzzle1_sum: {}", puzzle1_sum);
    println!("puzzle2_sum: {}", puzzle2_sum);
}
