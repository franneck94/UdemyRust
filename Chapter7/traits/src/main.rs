trait Summary {
    fn summarize(&self) -> String;
}

struct TwitterTweet {
    author: String,
    text: String,
}

struct YoutubeVideo {
    author: String,
    name: String,
    descr: String,
}

impl Summary for TwitterTweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.text)
    }
}

impl Summary for YoutubeVideo {
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.descr)
    }
}

fn notify(item: &impl Summary) {
    println!("{}", item.summarize());
}

fn notify2<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

fn main() {
    let tweet = TwitterTweet {
        author: String::from("Jan"),
        text: String::from("blabla;"),
    };
    println!("{}", tweet.summarize());
    notify(&tweet);

    let video = YoutubeVideo {
        author: String::from("Jan"),
        name: String::from("blub;"),
        descr: String::from("blabla;"),
    };
    println!("{}", video.summarize());
    notify(&video);
}
