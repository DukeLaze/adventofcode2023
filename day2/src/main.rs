#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    draws: Vec<Draw>,
}

fn parse_input(input: &str) -> Vec<Game> {
    // Example
    // Game 1: 7 green, 14 red, 5 blue; 8 red, 4 green; 6 green, 18 red, 9 blue
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue

    let mut games = Vec::new();

    // Split input into lines
    let lines: Vec<&str> = input.lines().collect();

    // Parse each line
    for line in lines {
        // ["Game X", "Draws colors;*]
        let parts = line.split(':').collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);
        // Get Game ID
        // Game X
        let id = parts[0].split_whitespace().nth(1).unwrap().trim();
        let id = id.parse::<u32>().unwrap();

        // Get Draws
        // ["Draws Colors,*"]
        let parts = parts[1].split(';').collect::<Vec<&str>>();

        let mut draws: Vec<Draw> = Vec::new();
        for draw in parts {
            let parts = draw.split(',').collect::<Vec<&str>>();
            // May be ',' or 'x color'. Does not need to contain all colors
            if parts.len() > 0 {
                let mut draw = Draw {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                // Take number and color
                let parts = parts
                    .into_iter()
                    .map(|part| part.trim())
                    .collect::<Vec<&str>>();
                for i in 0..parts.len() {
                    // count color
                    let parts = parts[i].split(' ').collect::<Vec<&str>>();
                    let num = parts[0].parse::<u32>().unwrap();
                    let color = parts[1];
                    if color == "red" {
                        draw.red = num
                    } else if color == "green" {
                        draw.green = num
                    } else if color == "blue" {
                        draw.blue = num
                    }
                }
                draws.push(draw);
            } else {
                draws.push(Draw {
                    red: 0,
                    green: 0,
                    blue: 0,
                })
            }
        }
        games.push(Game { id, draws });
    }
    games
}

fn is_legal_game(game: &Game) -> bool {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    game.draws
        .iter()
        .all(|draw| draw.red <= MAX_RED && draw.green <= MAX_GREEN && draw.blue <= MAX_BLUE)
}

fn get_power_of_minimum_required_colors_for_game(game: &Game) -> u32 {
    // Find the minimum number for each color
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for draw in &game.draws {
        min_red = std::cmp::max(min_red, draw.red);
        min_green = std::cmp::max(min_green, draw.green);
        min_blue = std::cmp::max(min_blue, draw.blue);
    }
    println!("min_red: {}, min_green: {}, min_blue: {} for game {:?}", min_red, min_green, min_blue, game);
    // Multiply together
    min_red * min_green * min_blue
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let games = parse_input(&input);

    // Part 1
    let mut id_sums = 0;
    for game in games.iter() {
        if is_legal_game(&game) {
            id_sums += game.id;
        }
    }

    // Part 2
    let mut power_sum = 0;
    for game in games.iter() {
        power_sum += get_power_of_minimum_required_colors_for_game(&game);
    }

    println!("id_sums: {}", id_sums);
    println!("power_sum: {}", power_sum);
}
