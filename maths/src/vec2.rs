
pub struct Vec2{
    pub x : f32,
    pub y : f32,
}

impl Vec2 {

    //Define Constants for commonly used vectors
    pub const ZERO : Self = Self { x: 0.0, y: 0.0};
    pub const X : Self = Self { x: 1.0, y: 0.0};
    pub const Y : Self = Self { x: 0.0, y: 1.0};

    pub fn new(x :f32, y : f32) -> Self {
        Self { x, y }
    }    
}