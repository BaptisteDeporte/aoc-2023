use reqwest::header::COOKIE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = std::env::var("SESSION")?;

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://adventofcode.com/2023/day/1/input")
        .header(COOKIE, ["session", &session].join("="))
        .send()?
        .text_with_charset("utf-8")?;
    let lines = resp.lines();
    // next step -> filter sur is_numeric, prendre les extreme
    let sum: i32 = lines
        .map(|line| {
            let mut line_chars = line.chars();
            let mut revert_line_chars = line.chars().rev();
            let first_digit_index = &line_chars
                .clone()
                .position(|char| char.is_numeric())
                .unwrap();
            let last_digit_index = &revert_line_chars
                .clone()
                .position(|char| char.is_numeric())
                .unwrap();

            let first_digit = line_chars.nth(*first_digit_index).unwrap();
            let last_digit = revert_line_chars.nth(*last_digit_index).unwrap();
            // return first_digit.unwrap();
            let mut digit_string = String::from(first_digit);
            digit_string.push(last_digit);
            let digit = i32::from_str_radix(&digit_string, 10);
            return digit.unwrap();
        })
        .sum();
    println!("{:?}", sum);
    Ok(())
}
