use crate::math::{Mat4, Vec3, Vec4};

#[derive(Clone, Copy)]
pub struct Position {
    vec: Vec3,
}

#[derive(Clone, Copy)]
pub struct Movement {
    vec: Vec3,
}

#[derive(Clone, Copy)]
pub struct Direction {
    vec: Vec3,
}

#[derive(Clone, Copy)]
pub struct LdrColor {
    r: f32,
    g: f32,
    b: f32,
}

#[derive(Clone, Copy)]
pub struct HdrColor {
    r: f32,
    g: f32,
    b: f32,
}

#[derive(Clone, Copy)]
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

    pub fn angle_between(&self, rhs: &Direction) -> f32 {
        self.vec.dot(rhs.vec)
    }

    pub fn perpendicular_to(&self, rhs: &Direction) -> Direction {
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
    camera: Box<dyn Camera>,
    world: Box<dyn Object>,
    lights: Vec<Box<dyn Light>>,
}

pub trait Camera {
    fn get_ray(&self, position_in_image: (f32, f32)) -> Ray;
}

pub struct Ray {
    origin: Position,
    direction: Direction,
}

pub struct OrthogonalCamera {
    position: Position,
    direction: Direction,
    size: (f32, f32),
    transform: Transform,
}

impl OrthogonalCamera {
    fn by_width(
        position: Position,
        yaw: f32,
        pitch: f32,
        roll: f32,
        aspect_ratio: f32,
        width: f32,
    ) -> OrthogonalCamera {
        let transform = Transform {
            mat: Mat4::rotate(yaw, pitch, roll),
        };
        let original_direction = Direction {
            vec: Vec3(0f32, 1f32, 0f32),
        };
        let direction = Direction {
            vec: (Vec4::from_movement(original_direction.vec) * transform.mat).into(),
        };

        OrthogonalCamera {
            position,
            direction,
            size: (width, width / aspect_ratio),
            transform,
        }
    }

    fn by_height(
        position: Position,
        yaw: f32,
        pitch: f32,
        roll: f32,
        aspect_ratio: f32,
        height: f32,
    ) -> OrthogonalCamera {
        let transform = Transform {
            mat: Mat4::rotate(yaw, pitch, roll),
        };
        let original_direction = Direction {
            vec: Vec3(0f32, 1f32, 0f32),
        };
        let direction = Direction {
            vec: (Vec4::from_movement(original_direction.vec) * transform.mat).into(),
        };

        OrthogonalCamera {
            position,
            direction,
            size: (height * aspect_ratio, height),
            transform,
        }
    }
}

impl Camera for OrthogonalCamera {
    fn get_ray(&self, position_in_image: (f32, f32)) -> Ray {
        let offset = Movement::new(
            self.size.0 * (position_in_image.0 * 2f32 - 1f32),
            0f32,
            self.size.1 * (position_in_image.1 * 2f32 - 1f32),
        );
        let rotated = offset * self.transform;
        let origin = self.position + rotated;
        Ray {
            origin,
            direction: self.direction,
        }
    }
}

pub struct PerspectiveCamera {
    position: Position,
    transform: Transform,
    tan_fov: (f32, f32),
}

impl PerspectiveCamera {
    fn by_x(
        position: Position,
        yaw: f32,
        pitch: f32,
        roll: f32,
        aspect_ratio: f32,
        fov_x: f32,
    ) -> PerspectiveCamera {
        let tan_fov_x = fov_x.tan();
        PerspectiveCamera {
            position,
            transform: Transform::rotate(yaw, pitch, roll),
            tan_fov: (tan_fov_x, tan_fov_x / aspect_ratio),
        }
    }

    fn by_y(
        position: Position,
        yaw: f32,
        pitch: f32,
        roll: f32,
        aspect_ratio: f32,
        fov_y: f32,
    ) -> PerspectiveCamera {
        let tan_fov_y = fov_y.tan();
        PerspectiveCamera {
            position,
            transform: Transform::rotate(yaw, pitch, roll),
            tan_fov: (tan_fov_y * aspect_ratio, tan_fov_y),
        }
    }
}

impl Camera for PerspectiveCamera {
    fn get_ray(&self, position_in_image: (f32, f32)) -> Ray {
        let direction = Direction::new(
            Movement::new(
                self.tan_fov.0 * (position_in_image.0 * 2f32 - 1f32),
                1f32,
                self.tan_fov.1 * (position_in_image.1 * 2f32 - 1f32),
            ) * self.transform,
        );
        return Ray {
            origin: self.position,
            direction,
        };
    }
}

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub struct Intersection {
    position: Position,
    real_normal: Direction,
    adjusted_normal: Direction,
    material: fn() -> Material,
}

pub struct Material {
    brdf: fn(viewer: Direction, light: Direction, normal: Direction) -> LdrColor,
}

pub trait Light {
    fn illuminate(self, adjusted_position: Position, world: &dyn Object) -> HdrColor;
}
