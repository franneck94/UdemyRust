# Summarization Trait Exercise

This exercise covers some topics that were not shown already in the course.  
Hence, getting to the solution by yourself may be hard to achieve.  
I suggest to watch my solution video.

## Objective

Implement a trait that defines a summarize method for summarizing posts on social media platforms.  
This exercise aims to reinforce understanding of traits, method implementations, and trait bounds in Rust.

This will be our social media post types:

```rust
struct FacebookPost {
    author: String,
    content: String,
}

struct InstagramPost {
    author: String,
    description: String,
}
```

## Instructions

### Define Trait

Create a trait named Summary with a method named summarize.  
The method should return a string representing a concise summary of the post.  
Provide a default implementation for the summarize method.  

### Implement Trait for Structs

Implement the Summary trait for the FacebookPost and InstagramPost structs. Override the summarize method for each struct to return a formatted summary including the author's name and the content or description of the post.

### Implement PartialEq and PartialOrd

Implement the PartialEq and PartialOrd traits for both FacebookPost and InstagramPost structs.  
Define equality and ordering based on the author's name.  

### Implement Eq

Implement the Eq trait for both FacebookPost and InstagramPost structs to ensure transitive equality.

### Main Function

```rust
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
```

#### Example Output

```text
Jan blabla
jan blabla
Jan image of blabla
jan image of blabla
IG1 == IG2 = false
FB1 == FB2 = false
IG1 < IG2 = false
FB1 < FB2 = true
```

Happy coding!
