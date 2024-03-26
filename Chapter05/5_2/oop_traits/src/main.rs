// Traits are similar to interfaces in other languages, although with some differences.

// trait Summary {
//     fn summarize(&self) -> String;
// }

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

// fn notify(post: &impl Summary) {
//     println!("{}", post.summarize());
// }

// fn notify<T: Summary>(post: &T) {
//     println!("{}", post.summarize());
// }

fn notify<T>(post: &T)
where
    T: Summary,
{
    println!("{}", post.summarize());
}

fn main() {
    let fb_post = FacebookPost {
        author: String::from("Jan"),
        content: String::from("blabla"),
    };
    println!("{}", fb_post.summarize());

    let ig_post = InstagramPost {
        author: String::from("Jan"),
        description: String::from("image of blabla"),
    };
    println!("{}", ig_post.summarize());

    notify(&fb_post);
    notify(&ig_post);
}
