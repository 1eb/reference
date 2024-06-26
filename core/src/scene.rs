pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Movement {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Direction {
    x: f32,
    y: f32,
    z: f32,
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
    mat: (
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
        (f32, f32, f32, f32),
    ),
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Position {
        Position { x, y, z }
    }
}

impl Movement {
    pub fn new(x: f32, y: f32, z: f32) -> Movement {
        Movement { x, y, z }
    }

    pub fn distance_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn distance(&self) -> f32 {
        self.distance_squared().sqrt()
    }
}

impl Direction {
    pub fn new(movement: Movement) -> Direction {
        let length = movement.distance();
        Direction {
            x: movement.x / length,
            y: movement.y / length,
            z: movement.z / length,
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
        Position::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<Position> for Position {
    type Output = Movement;

    fn sub(self, rhs: Position) -> Movement {
        Movement::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<f32> for Movement {
    type Output = Movement;

    fn mul(self, rhs: f32) -> Self::Output {
        Movement::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Mul<f32> for Direction {
    type Output = Movement;

    fn mul(self, rhs: f32) -> Self::Output {
        Movement::new(self.x * rhs, self.y * rhs, self.z * rhs)
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
