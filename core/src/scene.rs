use crate::{
    base_types::{Direction, HdrColor, LdrColor, Movement, Position, Transform},
    math::{Mat4, Vec3, Vec4},
};

pub struct Scene {
    pub camera: Box<dyn Camera>,
    pub world: Box<dyn Object>,
    pub ambient_light: HdrColor,
    pub lights: Vec<Box<dyn Light>>,
}

pub trait Camera {
    fn get_ray(&self, position_in_image: (f32, f32)) -> Ray;
    fn position(&self) -> Position;
}

pub struct Ray {
    pub origin: Position,
    pub direction: Direction,
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

    fn position(&self) -> Position {
        self.position
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
        let direction = Direction::from_movement(
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

    fn position(&self) -> Position {
        self.position
    }
}

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub struct Intersection<'a> {
    pub position: Position,
    pub real_normal: Direction,
    pub adjusted_normal: Direction,
    pub material: Box<dyn MaterialWrapper + 'a>,
}

pub trait MaterialWrapper {
    fn get_material(&self) -> Material;
}

pub struct Material {
    pub albedo: LdrColor,
    pub roughness: f32,
    pub f0: f32,
}

const F0_NORMAL: f32 = 0.04f32;
const F0_GOLD: f32 = 0.75f32;
const F0_SILVER: f32 = 0.97f32;
const F0_COPPER: f32 = 0.83f32;

pub trait Light {
    fn illuminate(
        &self,
        adjusted_position: Position,
        world: &dyn Object,
    ) -> Option<(HdrColor, Direction)>;
}

pub struct Plane {
    pub transform: Transform,
    pub coefficient_x0y0z0: f32,
    pub coefficient_x1y0z0: f32,
    pub coefficient_x0y1z0: f32,
    pub coefficient_x0y0z1: f32,
    pub material: Box<dyn Fn(Position) -> Material>,
}

impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        struct PlaneMaterialWrapper<'a> {
            parent: &'a Plane,
            position: Position,
        }

        impl<'a> MaterialWrapper for PlaneMaterialWrapper<'a> {
            fn get_material(&self) -> Material {
                (*self.parent.material)(self.position)
            }
        }

        let Ray { origin, direction } = *ray;
        let Position { vec: Vec3(p, q, r) } = origin;
        let Direction { vec: Vec3(u, v, w) } = direction;

        let mut coefficient_t1 = 0f32;
        let mut coefficient_t0 = 0f32;

        {
            coefficient_t0 += self.coefficient_x0y0z0;
        }
        {
            coefficient_t1 += self.coefficient_x1y0z0 * u;
            coefficient_t0 += self.coefficient_x1y0z0 * p;
        }
        {
            coefficient_t1 += self.coefficient_x0y1z0 * v;
            coefficient_t0 += self.coefficient_x0y1z0 * q;
        }
        {
            coefficient_t1 += self.coefficient_x0y0z1 * w;
            coefficient_t0 += self.coefficient_x0y0z1 * r;
        }

        let t = -coefficient_t0 / coefficient_t1;

        if t < 0f32 {
            None
        } else {
            let position = origin + direction * t;

            Some(Intersection {
                position,
                real_normal: Direction::from_movement(Movement::new(0f32, 1f32, 0f32)),
                adjusted_normal: Direction::from_movement(Movement::new(0f32, 1f32, 0f32)),
                material: Box::new(PlaneMaterialWrapper {
                    parent: &self,
                    position,
                }),
            })
        }
    }
}
