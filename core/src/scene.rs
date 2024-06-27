struct Vec3(f32, f32, f32);

struct Vec4(f32, f32, f32, f32);

struct Mat3(Vec3, Vec3, Vec3);

struct Mat4(Vec4, Vec4, Vec4, Vec4);

impl From<Vec4> for Vec3 {
    fn from(vec: Vec4) -> Self {
        Vec3(vec.0, vec.1, vec.2)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Vec3 {
    fn dot(self, rhs: Vec3) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    fn length_squared(self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    fn normalize(self) -> Vec3 {
        self / self.length()
    }
}

impl Vec4 {
    //
}

pub struct Position {
    vec: Vec3,
}

pub struct Movement {
    vec: Vec3,
}

pub struct Direction {
    vec: Vec3,
}

pub struct LdrColor {
    r: f32,
    g: f32,
    b: f32,
}

pub struct HdrColor {
    r: f32,
    g: f32,
    b: f32,
}

pub struct Transform {
    mat: Mat4,
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

    pub fn distance_squared(&self) -> f32 {
        self.vec.length_squared()
    }

    pub fn distance(&self) -> f32 {
        self.vec.length()
    }
}

impl Direction {
    pub fn new(movement: Movement) -> Direction {
        Direction {
            vec: movement.vec.normalize(),
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

impl std::ops::Mul<f32> for Movement {
    type Output = Movement;

    fn mul(self, rhs: f32) -> Self::Output {
        Movement {
            vec: self.vec * rhs,
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

pub struct Scene {
    camera: AnyCamera,
}

pub trait Camera {
    fn get_ray(position_in_image: (f32, f32)) -> Ray;
}

pub struct Ray {
    origin: Position,
    direction: Direction,
}

pub enum AnyCamera {
    Orthogonal(OrthogonalCamera),
    Perspective(PerspectiveCamera),
}

pub struct OrthogonalCamera {
    position: Position,
    direction: Direction,
    size: (f32, f32),
    transform: Transform,
}

pub struct PerspectiveCamera {
    position: Position,
    direction: Direction,
    tan_fov: (f32, f32),
}
