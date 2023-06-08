pub enum Button {
    Left,
    Right,
    Middle,
}

pub enum Event {
    KeyPress(u32),
    KeyRelease(u32),
    PointerDown(Vec2<f32>,Button),
    PointerUp(Vec2<f32>,Button),
    PointerMove(Vec2<f32>),
    PointerWheel(Vec2<f32>,Vec2<f32>),
    Configure(Rect<i32>),
    Expose(Rect<i32>),
    Close,
}

mod system;
pub use system::*;

mod window;
pub use window::*;
