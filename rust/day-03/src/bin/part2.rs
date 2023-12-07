fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output.unwrap());
}

#[derive(Debug)]
struct Datapoint<T> {
    x: usize,
    y: usize,
    data: T,
}

impl Datapoint<usize> {
    fn get_bounding_rect(&self) -> ((usize, usize), (usize, usize)) {
        let data_len = self.data.to_string().len();

        ((self.x, self.y), (self.x + data_len, self.y + 1))
    }
}

impl Datapoint<char> {
    fn get_bounding_rect(&self) -> ((usize, usize), (usize, usize)) {
        ((self.x, self.y), (self.x + 1, self.y + 1))
    }

    fn is_adjacent(&self, other: &Datapoint<usize>) -> bool {
        let sbr = self.get_bounding_rect();
        let obr = other.get_bounding_rect();
        !(sbr.0 .0 > obr.1 .0 || sbr.1 .0 < obr.0 .0 || sbr.0 .1 > obr.1 .1 || sbr.1 .1 < obr.0 .1)
    }
}

#[derive(Debug)]
enum Thing {
    Number(Datapoint<usize>),
    Symbol(Datapoint<char>),
}

impl Thing {
    fn gear_ratio(&self, all_things: &Vec<Thing>) -> Option<usize> {
        let symbol_dp = match self {
            Thing::Symbol(x) if x.data == '*' => x,
            _ => return None,
        };
        let numbers: Vec<_> = all_things.into_iter()
            .filter_map(|thing| match thing {
                Thing::Number(x) => Some(x),
                _ => None,
            })
            .into_iter()
            .filter_map(
                |number| match symbol_dp.is_adjacent(&number) {
                    true => Some(number.data),
                    false => None,
                },
            )
            .collect();
        match numbers.len() {
            2 => Some(numbers[0] * numbers[1]),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Schematic {
    things: Vec<Thing>,
    max_x: usize,
    max_y: usize,
}

impl Schematic {
    fn new(max_x: usize, max_y: usize) -> Schematic {
        Schematic {
            things: vec![],
            max_x,
            max_y,
        }
    }

    fn add_thing(&mut self, thing: Thing) {
        self.things.push(thing);
    }

    fn sum_of_parts(&self) -> usize {
        let symbols: Vec<_> = self
            .things
            .iter()
            .filter_map(|thing| match thing {
                Thing::Symbol(x) => Some(x),
                _ => None,
            })
            .collect();
        let numbers: Vec<_> = self
            .things
            .iter()
            .filter_map(|thing| match thing {
                Thing::Number(x) => Some(x),
                _ => None,
            })
            .into_iter()
            .filter_map(
                |number| match symbols.iter().any(|symbol| symbol.is_adjacent(number)) {
                    true => Some(number.data),
                    false => None,
                },
            )
            .collect();
        return numbers.iter().sum();
    }

    fn gear_ratio(&self) -> usize {
        self.things.iter().filter_map(|thing| thing.gear_ratio(&self.things)).sum()
    }
}

fn process(input: &str) -> Result<usize, &str> {
    let schematic = parse_schematic(input)?;
    // let sum_of_parts = schematic.sum_of_parts();
    Ok(schematic.gear_ratio())
}

fn parse_schematic(input: &str) -> Result<Schematic, &str> {
    let max_x = input
        .lines()
        .next()
        .ok_or("Error parsing schmatic: input is empty.")?
        .len()
        - 1;
    let max_y = input.lines().count() - 1;
    let mut schematic = Schematic::new(max_x, max_y);
    let mut number_buffer: Vec<char> = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char.is_ascii_digit() {
                number_buffer.push(char);
            } else {
                if !number_buffer.is_empty() {
                    let number = number_buffer
                        .iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .or(Err("Error parsing schematic: encountered invalid number."))?;
                    schematic.add_thing(Thing::Number(Datapoint {
                        x: x - number_buffer.len(),
                        y,
                        data: number,
                    }));
                    number_buffer.clear();
                }

                if char != '.' {
                    schematic.add_thing(Thing::Symbol(Datapoint { x, y, data: char }));
                }
            }
        }
        if !number_buffer.is_empty() {
            let number = number_buffer
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .or(Err("Error parsing schematic: encountered invalid number."))?;
            schematic.add_thing(Thing::Number(Datapoint {
                x: (line.len() - 1) - number_buffer.len(),
                y,
                data: number,
            }));
            number_buffer.clear();
        }
    }
    Ok(schematic)
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = process(input);
        assert_eq!(output.unwrap(), 467835);
    }
}
