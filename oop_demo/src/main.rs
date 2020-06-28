mod model;

fn main() {
    let mut post = model::Post::new();
    post.add_text("hello");
    post.request_review();
    post.approve();
    println!("aaaa {}", post.content());
}
