use std::io;

struct PromptLength<'a> {
    word_count: &'a String,
    lines_count: &'a String,
    character_count: &'a String,
}

impl<'a> PromptLength<'a> {
 
    fn read_input() -> String {
        let mut input = String::new();
    
        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        input
    }   

    fn word_count(&self) -> usize {
        self.word_count.split_whitespace().count()
    }

    fn lines_count(&self) -> usize{
        self.lines_count.lines().count()
    }

    fn character_count(&self) -> usize {
        self.character_count.chars().count()
    }
}


fn main() {
        let input = PromptLength::read_input();
    
//    loop {
        let prompt = PromptLength {
            word_count: &input,
            lines_count: &input,
            character_count: &input,
        };

        let word_count = prompt.word_count();
    
        let lines_count = prompt.lines_count();

        let character_count = prompt.character_count();

        println!("\n\n\nWord count>> {:?},\n\nlines count>> {:?},\n\ncharacters count>> {:?}", word_count, lines_count, character_count);
  //  }
}
