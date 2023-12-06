use std::{collections::HashMap, ops::Add};

#[derive(Debug)]
struct Draw {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    draws: Vec<Draw>,
}

impl Draw {
    fn new(red: i32, green: i32, blue: i32) -> Self {
        Draw { red, green, blue }
    }
}

impl Game {
    fn new(id: i32, draws: Vec<Draw>) -> Self {
        Game { id, draws }
    }
}

fn main() {
    let res: i32 = include_str!("../input.txt")
        .split("\n")
        .map(|line| line_to_game(String::from(line)))
        .map(|game| {
            let draws = game.draws;
            let max_red = draws.iter().max_by(|a, b| a.red.cmp(&b.red)).unwrap().red;
            let max_green = draws
                .iter()
                .max_by(|a, b| a.green.cmp(&b.green))
                .unwrap()
                .green;
            let max_blue = draws
                .iter()
                .max_by(|a, b| a.blue.cmp(&b.blue))
                .unwrap()
                .blue;
            return max_red * max_green * max_blue;
        })
        .sum();

    println!("{}", res);
}

fn line_to_game(line: String) -> Game {
    let line_vector: Vec<&str> = line.split(": ").collect();
    let [raw_id, raw_values]: [&str; 2] = line_vector.try_into().unwrap();

    let id = i32::from_str_radix(&raw_id.replace("Game ", ""), 10).unwrap();

    let raw_draws: Vec<&str> = raw_values.split("; ").collect();

    let mut draws: Vec<Draw> = Vec::new();

    for raw_draw in raw_draws {
        let mut values = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        let raw_values: Vec<&str> = raw_draw.split(", ").collect();

        for raw_value in raw_values {
            let value_vector: Vec<&str> = raw_value.split(" ").collect();
            let [count, color]: [&str; 2] = value_vector.try_into().unwrap();
            values.insert(
                color,
                values[color].add(i32::from_str_radix(count, 10).unwrap()),
            );
        }

        let draw = Draw::new(values["red"], values["green"], values["blue"]);
        draws.push(draw);
    }
    return Game::new(id, draws);
}
