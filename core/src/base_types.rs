use crate::math::{Mat4, Vec3, Vec4};

#[derive(Clone, Copy)]
pub struct Position {
    pub vec: Vec3,
}

#[derive(Clone, Copy)]
pub struct Movement {
    pub vec: Vec3,
}

#[derive(Clone, Copy)]
pub struct Direction {
    pub vec: Vec3,
}

#[derive(Clone, Copy)]
pub struct LdrColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Copy)]
pub struct HdrColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

#[derive(Clone, Copy)]
pub struct Transform {
    pub mat: Mat4,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Position {
        Position { vec: Vec3(x, y, z) }
    }
}

impl Movement {
    pub fn new(x: f32, y: f32, z: f32) -> Movement {
        Movement { vec: Vec3(x, y, z) }
    }

    pub fn distance_squared(self) -> f32 {
        self.vec.length_squared()
    }

    pub fn distance(self) -> f32 {
        self.vec.length()
    }
}

impl Direction {
    pub fn from_movement(movement: Movement) -> Direction {
        Direction {
            vec: movement.vec.normalize(),
        }
    }

    pub fn from_directions<const T: usize>(directions: [Direction; T]) -> Direction {
        Direction {
            vec: directions
                .iter()
                .fold(Vec3(0f32, 0f32, 0f32), |acc, Direction { vec }| acc + *vec)
                .normalize(),
        }
    }

    pub fn cos_angle_between(self, rhs: Direction) -> f32 {
        self.vec.dot(rhs.vec)
    }

    pub fn perpendicular_to(self, rhs: Direction) -> Direction {
        Direction {
            vec: self.vec.cross(rhs.vec),
        }
    }
}

impl Transform {
    pub fn inverse(self) -> Transform {
        Transform {
            mat: self.mat.inverse(),
        }
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Transform {
        Transform {
            mat: Mat4::translate(x, y, z),
        }
    }

    pub fn rotate_x_by_trigonometric(cos: f32, sin: f32) -> Transform {
        Transform {
            mat: Mat4::rotate_x_by_trigonometric(cos, sin),
        }
    }

    pub fn rotate_y_by_trigonometric(cos: f32, sin: f32) -> Transform {
        Transform {
            mat: Mat4::rotate_y_by_trigonometric(cos, sin),
        }
    }

    pub fn rotate_z_by_trigonometric(cos: f32, sin: f32) -> Transform {
        Transform {
            mat: Mat4::rotate_z_by_trigonometric(cos, sin),
        }
    }

    pub fn rotate_x_by_angle(angle: f32) -> Transform {
        Transform {
            mat: Mat4::rotate_x_by_angle(angle),
        }
    }

    pub fn rotate_y_by_angle(angle: f32) -> Transform {
        Transform {
            mat: Mat4::rotate_y_by_angle(angle),
        }
    }

    pub fn rotate_z_by_angle(angle: f32) -> Transform {
        Transform {
            mat: Mat4::rotate_z_by_angle(angle),
        }
    }

    pub fn rotate(yaw: f32, pitch: f32, roll: f32) -> Transform {
        Transform {
            mat: Mat4::rotate(yaw, pitch, roll),
        }
    }

    pub fn scale3(x: f32, y: f32, z: f32) -> Transform {
        Transform {
            mat: Mat4::scale3(x, y, z),
        }
    }

    pub fn scale1(scale: f32) -> Transform {
        Transform {
            mat: Mat4::scale1(scale),
        }
    }
}

impl LdrColor {
    pub fn new(r: f32, g: f32, b: f32) -> LdrColor {
        LdrColor {
            r: LdrColor::in_range(r),
            g: LdrColor::in_range(g),
            b: LdrColor::in_range(b),
        }
    }

    fn in_range(f: f32) -> f32 {
        if f < 0f32 {
            0f32
        } else if f > 1f32 {
            1f32
        } else {
            f
        }
    }
}

impl HdrColor {
    pub fn new(r: f32, g: f32, b: f32) -> HdrColor {
        HdrColor {
            r: HdrColor::in_range(r),
            g: HdrColor::in_range(g),
            b: HdrColor::in_range(b),
        }
    }

    fn in_range(f: f32) -> f32 {
        if f < 0f32 {
            0f32
        } else {
            f
        }
    }
}

impl std::ops::Add<Movement> for Position {
    type Output = Position;

    fn add(self, rhs: Movement) -> Position {
        Position {
            vec: self.vec + rhs.vec,
        }
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Movement;

    fn sub(self, rhs: Position) -> Movement {
        Movement {
            vec: self.vec - rhs.vec,
        }
    }
}

impl Position {
    fn apply(self, transform: Transform) -> Position {
        Position {
            vec: (Vec4::from_movement(self.vec) * transform.mat).into(),
        }
    }
}

impl std::ops::Mul<f32> for Movement {
    type Output = Movement;

    fn mul(self, rhs: f32) -> Self::Output {
        Movement {
            vec: self.vec * rhs,
        }
    }
}

impl std::ops::Mul<Transform> for Movement {
    type Output = Movement;

    fn mul(self, rhs: Transform) -> Self::Output {
        Movement {
            vec: (Vec4::from_movement(self.vec) * rhs.mat).into(),
        }
    }
}

impl std::ops::Neg for Movement {
    type Output = Movement;

    fn neg(self) -> Movement {
        Movement {
            vec: self.vec * -1f32,
        }
    }
}

impl std::ops::Mul<f32> for Direction {
    type Output = Movement;

    fn mul(self, rhs: f32) -> Movement {
        Movement {
            vec: self.vec * rhs,
        }
    }
}

impl std::ops::Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        Direction {
            vec: self.vec * -1f32,
        }
    }
}

impl std::ops::Mul<f32> for LdrColor {
    type Output = HdrColor;

    fn mul(self, rhs: f32) -> HdrColor {
        HdrColor::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl std::ops::Mul<HdrColor> for LdrColor {
    type Output = HdrColor;

    fn mul(self, rhs: HdrColor) -> HdrColor {
        HdrColor::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl From<LdrColor> for HdrColor {
    fn from(value: LdrColor) -> Self {
        HdrColor::new(value.r, value.g, value.b)
    }
}

impl std::ops::Mul<f32> for HdrColor {
    type Output = HdrColor;

    fn mul(self, rhs: f32) -> HdrColor {
        HdrColor::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl std::ops::Mul<HdrColor> for HdrColor {
    type Output = HdrColor;

    fn mul(self, rhs: HdrColor) -> HdrColor {
        HdrColor::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl std::ops::Add<HdrColor> for HdrColor {
    type Output = HdrColor;

    fn add(self, rhs: HdrColor) -> HdrColor {
        HdrColor::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}
