use std::cmp::{Eq, PartialEq, PartialOrd};

trait Summary {
    fn summarize(&self) -> String {
        format!("Not Implemented!")
    }
}

struct FacebookPost {
    author: String,
    content: String,
}

struct InstagramPost {
    author: String,
    description: String,
}

impl Summary for FacebookPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.content)
    }
}

impl Summary for InstagramPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.description)
    }
}

impl PartialEq for FacebookPost {
    fn eq(&self, other: &Self) -> bool {
        self.author == other.author && self.content == other.content
    }
}

impl PartialEq for InstagramPost {
    fn eq(&self, other: &Self) -> bool {
        self.author == other.author && self.description == other.description
    }
}

impl PartialOrd for FacebookPost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // lexicographical ordering
        Some(self.author.cmp(&other.author))
    }
}

impl PartialOrd for InstagramPost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // lexicographical ordering
        Some(self.author.cmp(&other.author))
    }
}

// a == b (PartialEq), b == c (PartialEq) => a == c (Eq)
impl Eq for FacebookPost {}

// a == b (PartialEq), b == c (PartialEq) => a == c (Eq)
impl Eq for InstagramPost {}

fn main() {
    let fb_post = FacebookPost {
        author: String::from("Jan"),
        content: String::from("blabla"),
    };
    println!("{}", fb_post.summarize());

    let fb_post2 = FacebookPost {
        author: String::from("jan"),
        content: String::from("blabla"),
    };
    println!("{}", fb_post2.summarize());

    let ig_post = InstagramPost {
        author: String::from("Jan"),
        description: String::from("image of blabla"),
    };
    println!("{}", ig_post.summarize());

    let ig_post2 = InstagramPost {
        author: String::from("jan"),
        description: String::from("image of blabla"),
    };
    println!("{}", ig_post2.summarize());

    println!("IG1 == IG2 = {}", ig_post == ig_post2);
    println!("FB1 == FB2 = {}", fb_post == fb_post2);

    println!("IG1 < IG2 = {}", ig_post < ig_post2);
    println!("FB1 < FB2 = {}", fb_post < fb_post2);
}
