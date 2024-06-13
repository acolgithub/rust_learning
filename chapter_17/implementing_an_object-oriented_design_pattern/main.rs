// include Post from blog crate
use blog::Post;

fn main() {
    // create new post
    let mut post = Post::new();

    // add text to blog post
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());  // should have no content since post is still a draft

    // request review of added post
    post.request_review();
    assert_eq!("", post.content());  // should have no content since post is still a draft

    // approve post
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());  // should have content since post is approved
}
