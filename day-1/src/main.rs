use reqwest::header::COOKIE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = std::env::var("SESSION")
        .map_err(|err| format!("Error while fetching \"SESSION\" var env : {}", err))
        .unwrap();

    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("https://adventofcode.com/2023/day/1/input")
        .header(COOKIE, ["session", &session].join("="))
        .send()?
        .text_with_charset("utf-8")?;
    let lines = resp.lines();
    // next step -> filter sur is_numeric, prendre les extreme
    let sum: u32 = lines
        .map(|line| {
            let mut numerics_values = line.chars().filter(|char| char.is_ascii_digit());
            let first_digit = match numerics_values.next() {
                Some(char_digit) => char_digit,
                None => panic!("None digit found for this line: {}", line),
            };
            let last_digit = numerics_values.last().unwrap_or(first_digit);
            let mut digit_string = String::from(first_digit);
            digit_string.push(last_digit);
            return u32::from_str_radix(&digit_string, 10).unwrap();
        })
        .sum();
    println!("Sum of all of the calibration values: {:?}", sum);
    Ok(())
}
