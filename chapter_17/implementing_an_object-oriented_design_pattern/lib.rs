// Post struct
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {

    // construct new Post instance
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {}));
            content: String::new(),
        }
    }

    // add text to Post
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // check content of Post, returns empty if not approved
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // request review of Post, should change its state
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {  // take used to extract the Some value and leave a None
            self.state = Some(s.request_review())
        }
    }

    // approve Post, should change its state
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// create State trait object to define behaviour that all state objects must have
trait State {

    // functions needed with State trait
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // implement content function default, published overrides with its own version
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// create Draft struct
struct Draft {}

impl State for Draft {
    // requesting review results ing PendingReview state
    fn request_review(self: Box<self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // approve preserves state when in draft
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// Pending Review struct
struct PendingReview {}

// requesting review when already PendingReview results in self
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // approve modifies state to published when pending review
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    // request review preserves state after published
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // approve preserves state after published
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // get content of published post
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
