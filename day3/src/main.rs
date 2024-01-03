/*
Rules:

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?
*/

fn extract_numbers(schematic: &str) -> Vec<u32> {
    // Create two-dimensional vector
    let schematic = schematic
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Print to veriy
    for row in 0..schematic.len() {
        for col in 0..schematic[row].len() {
            print!("{}", schematic[row][col]);
        }
        println!();
    }

    println!("\n 4 first character {}{}{}{}", schematic[0][0], schematic[0][1], schematic[0][2], schematic[0][3]);  
    let mut numbers = Vec::new();

    // Find all numbers, exclude if surrounded (including diagonally) by '.'
    for row in 0..schematic.len() {
        let mut col = 0;
        while col < schematic[row].len() {
            println!("row: {}, col {}", row, col);
            if !schematic[row][col].is_digit(10) {
                col += 1;
                continue;
            }
            if schematic[row][col].is_digit(10) {
               let (is_valid, num) = part_number(&schematic, row, col);
                if is_valid{
                    numbers.push(num);
                }
                col += num.to_string().len();
                continue;
            }

            col += 1;
        }
    }

    println!("Numbers: {:?}", numbers);
    numbers

}

fn part_number(schematic: &Vec<Vec<char>>, mut row: usize, mut col: usize) -> (bool, u32) {
    let mut is_valid = false;

    // Extract number
    let mut num = String::new();
    {
        let mut col = col;
        num.push(schematic[row][col]);
        loop {
            // Break if edge
            col += 1;
            if col == schematic[row].len() {
                break;
            }
            if schematic[row][col].is_numeric() {
                num.push(schematic[row][col]);
            
                continue;
            }

            break;
        }
    }

    let num = num.parse::<u32>().unwrap();

    
    // Check if part number is valid
    if col == 0 {
        col = 1;
    }

    if row == 0 {
        row = 1;
    }
    
    for i in row-1..=row+1 {
        for j in col-1..=col+num.to_string().len() {
            // Stop if reached edge
            if i >= schematic.len() || j >= schematic[i].len() {
                break;
            }
            if schematic[i][j] == '.' {
                continue;
            }
            if !schematic[i][j].is_digit(10) {
                is_valid = true;
                break;
            }
        }
    }
    println!("Number at ({},{}) is {} and is valid: {}", row, col, num, is_valid);

    (is_valid, num)
}

fn main() {
    let schematic = include_str!("input.txt");
    let numbers = extract_numbers(schematic);
    println!("Schematic sum: {}", numbers.iter().sum::<u32>());
}
