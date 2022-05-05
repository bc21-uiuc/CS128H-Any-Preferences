use std::io::{BufRead, BufReader};
use std::fs::File;
use vader_sentiment;
use rand::Rng;

//idea get sentiment of sentence and use that as a factor for all the words in that sentence
// also gotta credit https://github.com/ckw017/vader-sentiment-rust
//idk can think about this later

pub fn giveInts() -> Vec<i32> {
    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();
    let reader = BufReader::new(File::open("src/test.html").expect("Cannot open file.txt"));
    let mut int_vector: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let word = line.unwrap().trim().to_string();
            let ye = middle(word);
            let score = (analyzer.polarity_scores(&ye)["compound"] * 100.0) as i32;
            let bin = match score {
                -100..=-60 => 0,
                -61..=-20 => 1,
                -21..=20 => 2,
                21..=60 => 3,
                61..=100 => 4,
                _ => 0
            };
            if ye.len() == 0 {
                continue;
            }
            let min_val = bin * 51;
            for individual in ye.split_whitespace() {
                int_vector.push(min_val + randomizer(individual.to_string()));
            }
    }
    int_vector
}


//either my random or compute bin function must give result between 0-51
fn randomizer(string: String) -> i32 {
    let mut vowel_count: i32 = 0;
    let mut upper: i32 = 0;

    for x in string.chars() {
        if x.is_ascii_uppercase() && upper <= 20 {
            upper += 5;
        }
        match x {
            'a' => vowel_count += 1,
            'e' => vowel_count += 1,
            'i' => vowel_count += 1,
            'o' => vowel_count += 1,
            'u' => vowel_count += 1,
            _ => ()
        };
    }
    let random: i32 = rand::thread_rng().gen_range(-33..34);
    let v_contribution = ((1 + vowel_count)  * 17) % 23;
    let result = random + upper + v_contribution;
    match result {
        0..=255 => result,
        _=> upper + v_contribution
    }
}


fn middle(text: String) -> String {
    if !text.contains('>') && !text.contains('<') {
        return text;
    }
    let mut tring = String::new();
    let pos = text.chars().position(|c| c == '>').unwrap() + 1;
    let test = &text[pos..];
    for x in test.chars() {
        if match x {
            '<' => true,
            '\n' => true,
            _ => false
        } {
            tring += "";
            break;
        }
        tring.push(x);
    }
    tring
}