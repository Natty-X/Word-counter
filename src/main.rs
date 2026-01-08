use std::io;

struct PromptLength {
    word_count: usize,
    lines_count: usize,
    character_count: usize,
}

impl PromptLength {
 
    fn read_input() -> String {
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        input
    }   

    fn word_count(&self, mut input: Vec<String>) -> usize {
        let prompt = Self::read_input();
        input.push(prompt);
        input.len()
    }
}


fn main() {
    println!("Hello, world!");
}
