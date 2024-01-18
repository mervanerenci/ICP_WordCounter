struct WordCounter {
    text: String,
}

impl WordCounter {
    fn new(text: &str) -> WordCounter {
        WordCounter {
            text: String::from(text),
        }
    }

    fn count_words(&self) -> Result<usize, &'static str> {
        if self.text.trim().is_empty() {
            Err("Empty string")
        } else {
            let words: Vec<&str> = self.text.split_whitespace().collect();
            Ok(words.len())
        }
    }
}

fn main() {
    println!("Enter a text:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let word_counter = WordCounter::new(&input);
    let word_count = word_counter.count_words();
    match word_count {
        Ok(count) => println!("Word count: {}", count),
        Err(err) => println!("Error: {}", err),
    }
}
