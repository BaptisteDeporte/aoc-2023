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
    let max_by_color = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let filtred_games = include_str!("../input.txt")
        .split("\n")
        .map(|line| {
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
        })
        .filter(|game| {
            !game.draws.iter().any(|draw| {
                draw.blue > max_by_color["blue"]
                    || draw.red > max_by_color["red"]
                    || draw.green > max_by_color["green"]
            })
        });
    for game in filtred_games.clone() {
        println!("{:?}", game);
    }

    let sum: i32 = filtred_games.map(|game| game.id).sum();
    println!("{}", sum);
}
