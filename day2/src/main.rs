struct Draw {
    red: u32,
    green: u32,
    blue: u32
}

struct Game {
    id: u32,
    draws: Vec<Draw>
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
        let parts = line.split(':').collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);
        // Get Game ID
        // Game X
        let id = parts[0].split_whitespace().nth(1).unwrap().trim();
        println!("id: {}", id);
        let id = id.parse::<u32>().unwrap();

        // Get Draws
        let parts = parts[1].split(';').collect::<Vec<&str>>();
        assert_eq!(parts.len(), 3);

        for draw in parts {
            let mut draws: Vec<Draw> = Vec::new();
            let parts = draw.split(',').collect::<Vec<&str>>();
            // May be ',' or 'x color'. Does not need to contain all colors
            if parts.len() > 0 && parts.len() % 2 == 0 {
                let mut draw = Draw {
                    red: 0,
                    green: 0,
                    blue: 0,
                };

                // Take number and color
                let parts = parts.chunks(2).collect::<Vec<&[&str]>>();
                for i in 0..parts.len() {
                    let color = parts[i][1];
                    let num = parts[i][0].parse::<u32>().unwrap();
                    if color == "red" { draw.red = num }
                    else if color == "green" { draw.green = num }
                    else if color == "blue" { draw.blue = num }
                }
            }
            else {
                draws.push(Draw {
                    red: 0,
                    green: 0,
                    blue: 0,
                })
            }

        games.push(Game { id, draws });
    }
    }
    games
}


fn is_legal_game(game: &Game) -> bool {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    
    game.draws.iter().all(|draw| draw.red <= MAX_RED && draw.green <= MAX_GREEN && draw.blue <= MAX_BLUE)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let games = parse_input(&input);

    let mut id_sums = 0;
    for game in games {
        if is_legal_game(&game) {
            id_sums += game.id;
        }
    }

    println!("{}", id_sums);
}
