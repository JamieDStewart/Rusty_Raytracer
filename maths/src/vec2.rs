#[derive(Copy, Clone, Default, PartialEq)]
pub struct Vec2{
    pub x : f32,
    pub y : f32,
}

pub const fn vec2( x: f32, y: f32) -> Vec2 {
    Vec2{ x, y}
}

impl Vec2 {

    //Define Constants for commonly used vectors
    pub const ZERO : Self = Self { x: 0.0, y: 0.0};
    pub const X : Self = Self { x: 1.0, y: 0.0};
    pub const Y : Self = Self { x: 0.0, y: 1.0};

    pub fn new(x :f32, y : f32) -> Self {
        Self { x, y }
    }  

    pub fn abs(self) -> Self {
        vec2( self.x.abs(), self.y.abs())
    }  
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add( self, _rhs: Vec2) -> Vec2 {
        Vec2::new( self.x + _rhs.x, self.y + _rhs.y )
    }
}

impl std::ops::AddAssign<Vec2> for Vec2 {
    
    fn add_assign( &mut self, rhs: Self) {
        *self = Self { 
            x: self.x + rhs.x,
            y: self.y + rhs.y
        };
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub( self, _rhs: Vec2) -> Vec2 {
        Vec2::new( self.x - _rhs.x, self.y - _rhs.y )
    }
}

impl std::ops::SubAssign<Vec2> for Vec2 {
    
    fn sub_assign( &mut self, rhs: Self) {
        *self = Self { 
            x: self.x - rhs.x,
            y: self.y - rhs.y
        };
    }
}

impl std::ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul( self, _rhs: Vec2) -> Vec2 {
        Vec2::new( self.x * _rhs.x, self.y * _rhs.y )
    }
}

impl std::ops::MulAssign<Vec2> for Vec2 {
    
    fn mul_assign( &mut self, rhs: Self) {
        *self = Self { x: self.x * rhs.x, y: self.y * rhs.y };
    }
}

//This will test for equality '==' operator. Runs comparison of all values in struct
impl std::cmp::Eq for Vec2 {}


impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(precision) = f.precision() {
            write!(f, "[{1:.0$} {2:.0$}]", precision, self.x, self.y)
        } else {
            write!(f, "[{:.1} {:.1}]", self.x, self.y)
        }
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        self.x.fmt(f)?;
        f.write_str(" ")?;
        self.y.fmt(f)?;
        f.write_str("]")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    //add in a macro rule for floating point errors to allow for almost equal
    macro_rules! almost_eq{
        ($left : expr, $right: expr) => {
            let left = $left;
            let right = $right;
            assert!((left - right).abs() < 1e-6, "{} != {}", left, right );
        };
    }

    #[test]
    fn test_vec2_assignment(){

        let a = vec2( 1.0, 2.0);
        assert_eq!( a, vec2(1.0, 2.0));
        let b = a;
        assert_eq!( b, a );
        let c = Vec2::new( 3.0, 1.7);
        assert_eq!( c, vec2( 3.0, 1.7));
    }

    #[test]
    fn test_vec2_operations(){
        let a = vec2( 1.0, 1.0) + vec2( 2.0, 2.0);
        assert_eq!( a, vec2( 3.0, 3.0));
        almost_eq!( a.x, 3.0);
        let mut b = vec2( 2.0, 2.0) - vec2( 1.0, 1.0);
        assert_eq!( b, vec2(1.0, 1.0));
        b += vec2( 1.0, 1.0);
        assert_eq!( b, vec2(2.0, 2.0));
        b -= vec2( 1.0, 1.0);
        assert_eq!( b, vec2(1.0, 1.0));
        let mut c = b * vec2( 2.0, 3.0);
        assert_eq!( c, vec2(2.0, 3.0));
        c *= vec2(2.0, 2.0);
        assert_eq!( c, vec2(4.0, 6.0));
    }
}