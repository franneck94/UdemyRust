# Summarization Trait Exercise

## Objective

Implement a trait that defines a summarize method for summarizing posts on social media platforms.  
This exercise aims to reinforce understanding of traits, method implementations, and trait bounds in Rust.

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
Provide a default implementation for the summarize method that returns "Not Implemented!".  

### Implement Trait for Structs

Implement the Summary trait for the FacebookPost and InstagramPost structs. Override the summarize method for each struct to return a formatted summary including the author's name and the content or description of the post.

### Implement PartialEq and PartialOrd

Implement the PartialEq and PartialOrd traits for both FacebookPost and InstagramPost structs.  
Define equality and ordering based on the author's name.  
Use lexicographical ordering for comparing authors' names.

### Implement Eq

Implement the Eq trait for both FacebookPost and InstagramPost structs to ensure transitive equality.

### Main Function

In the main function, create instances of FacebookPost and InstagramPost.  
Print the summaries of each post to verify the correctness of the summarize method implementation.  
Additionally, compare two posts of each type for equality and ordering using the implemented traits.

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
