use std::io::stdin;
trait Getinfo {
    fn get_info(&self) -> String;
}

struct Message {
    author: String,
    text: String,
}

struct Post {
    author: String,
    content: String,
}

impl Getinfo for Message {
    fn get_info(&self) -> String {
        return format!("Message author is: {}\nMessage text is: {}", self.author, self.text);
    }
}

impl Getinfo for Post {
    fn get_info(&self) -> String {
        return format!("Post author is: {}\nPost content: {}", self.author, self.content);
    }
} 
fn main() {
    let mut message_author: String = String::new();
    let mut post_author: String = String::new();
    let mut message_text: String = String::new();
    let mut post_text: String = String::new();

    println!("Enter a message author");
    stdin().read_line(&mut message_author).expect("Error getting a user input");
    println!("Enter a message text");
    stdin().read_line(&mut message_text).expect("Error getting a user input");
    println!("Enter a post author");
    stdin().read_line(&mut post_author).expect("Error getting a user input");
    println!("Enter a post_text");
    stdin().read_line(&mut post_text).expect("Error getting a user input");


    let Message_by = Message {
        author: message_author.to_string(),
        text: message_text.to_string(),
    };

    let post_by = Post {
        author: post_author.to_string(),
        content: post_text.to_string(),
        Likes: 11,
    };
    println!("-------------- <_> -------------");
    println!("You have 2 unreaded message!...");
    println!("-------------- <_> -------------");
    println!("{}", Message_by.get_info());
    println!("-------------- <_> -------------");
    println!("{}", post_by.get_info());
    println!("-------------- <_> -------------");


}