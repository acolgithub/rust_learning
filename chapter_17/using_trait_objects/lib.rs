// Draw trait with single method
pub trait Draw {
    fn draw(&self);
}

// Screen struct with components field which holds a vector of trait objects implementing Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

// run method which calls the draw method on each of its components
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Screen struct with generic type and trait bound
pub struct Scree2<T: Draw> {
    pub components: Vec<T>
}

// implementation of run method with generic type and trait bound.
// all components must have same type.
impl<T> Screen<T>
where
    T: Draw
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// implementation of Button struct
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

// implementation of Draw method for Button
impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

// implementation of SelectBox struct
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

// implementation of Draw method for SelectBox
impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}
