fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
struct Game {
    id: u32,
    max_blue: Option<u32>,
    max_green: Option<u32>,
    max_red: Option<u32>,
}

impl Game {
    fn new(id: u32) -> Game {
        Game {
            id,
            max_blue: None,
            max_green: None,
            max_red: None,
        }
    }

    fn update(&mut self, color: &str, amount: u32) {
        match color {
            "blue" if !self.max_blue.is_some_and(|current| current > amount) => {
                self.max_blue = Some(amount)
            }
            "green" if !self.max_green.is_some_and(|current| current > amount) => {
                self.max_green = Some(amount)
            }
            "red" if !self.max_red.is_some_and(|current| current > amount) => {
                self.max_red = Some(amount)
            }
            _ => {}
        }
    }

    fn is_invalid(&self) -> bool {
        self.max_blue.is_some_and(|amount| amount > MAX_BLUE)
            || self.max_green.is_some_and(|amount| amount > MAX_GREEN)
            || self.max_red.is_some_and(|amount| amount > MAX_RED)
    }

    fn power(&self) -> u32 {
        return self.max_blue.unwrap_or_default()
            * self.max_green.unwrap_or_default()
            * self.max_red.unwrap_or_default();
    }
}

fn parse_game(line: &str) -> Result<Game, &str> {
    let mut splits = line.split(":");

    let game_info = splits.next().ok_or("Error parsing game: line seems to be empty")?;
    let draw_info = splits.next().ok_or("Error parsing game: line had no draws.")?;

    // If there's still splits left
    // the line is incomprehensible nonsense.
    // ...is there more idiomatic way to check if iterator is exhausted?
    if splits.any(|_| true) {
        return Err("Error parsing game: line contains too many colons.");
    }

    let mut game: Game = game_info
        .split(" ")
        .last()
        .ok_or("Error parsing game: could not parse game id.")?
        .parse::<u32>()
        .and_then(|game_id| Ok(Game::new(game_id)))
        .or(Err("Error parsing game: invalid game id."))?;

    draw_info
        .split(";")
        .map(|draw| draw.split(","))
        .flatten()
        .filter_map(|cube_info| {
            let mut info_parts = cube_info.trim().split(" ");
            let amount = info_parts.next()?.trim().parse::<u32>().ok()?;
            let color = info_parts.next()?.trim();
            match info_parts.next() {
                None => Some((color, amount)),
                Some(_) => None,
            }
        })
        .for_each(|(color, amount)| game.update(color, amount));

    Ok(game)
}

fn process(input: &str) -> Result<u32, &str> {
    let games: Vec<Game> = match input.lines().map(parse_game).collect() {
        Err(err) => return Err(err),
        Ok(games) => games,
    };
    Ok(games.into_iter().map(|game| game.power()).sum())
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = process(input);
        assert_eq!(output.unwrap(), 2286);
    }
}
