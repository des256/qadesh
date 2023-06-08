pub trait RenderObject {
    void layout(&self);
}

pub trait Element {

}

pub trait Widget {
    Box<dyn Element> update(&self,Option<Box<dyn Element>>);
}

mod text;
pub use text::*;

mod button;
pub use button::*;

mod column;
pub use column::*;
