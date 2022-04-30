trait Summary {
    fn summarize(&self) -> String {
        format!("Empty...")
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

fn notify(element: &impl Summary) {
    println!("{}", element.summarize());
}

fn main() {
    let fb_post = FacebookPost {
        author: String::from("Jan"),
        content: String::from("blabla"),
    };

    let ig_post = InstagramPost {
        author: String::from("Jan"),
        description: String::from("also blabla"),
    };

    notify(&fb_post);
    notify(&ig_post);
}
