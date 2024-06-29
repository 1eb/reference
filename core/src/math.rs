#[derive(Clone, Copy)]
pub struct Vec3(pub f32, pub f32, pub f32);

#[derive(Clone, Copy)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

#[derive(Clone, Copy)]
pub struct Mat4(pub Vec4, pub Vec4, pub Vec4);

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
    pub fn dot(self, rhs: Vec3) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn inverse(self) -> Vec3 {
        Vec3(1f32 / self.0, 1f32 / self.1, 1f32 / self.2)
    }

    pub fn length_squared(self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }
}

impl Vec4 {
    pub fn from_position(position: Vec3) -> Vec4 {
        Vec4(position.0, position.1, position.2, 1f32)
    }

    pub fn from_movement(movement: Vec3) -> Vec4 {
        Vec4(movement.0, movement.1, movement.2, 0f32)
    }
}

impl std::ops::Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4(
            Vec4(
                self.0 .0 * rhs.0 .0 + self.0 .1 * rhs.1 .0 + self.0 .2 * rhs.2 .0,
                self.0 .0 * rhs.0 .1 + self.0 .1 * rhs.1 .1 + self.0 .2 * rhs.2 .1,
                self.0 .0 * rhs.0 .2 + self.0 .1 * rhs.1 .2 + self.0 .2 * rhs.2 .2,
                self.0 .0 * rhs.0 .3 + self.0 .1 * rhs.1 .3 + self.0 .2 * rhs.2 .3 + self.0 .3,
            ),
            Vec4(
                self.1 .0 * rhs.0 .0 + self.1 .1 * rhs.1 .0 + self.1 .2 * rhs.2 .0,
                self.1 .0 * rhs.0 .1 + self.1 .1 * rhs.1 .1 + self.1 .2 * rhs.2 .1,
                self.1 .0 * rhs.0 .2 + self.1 .1 * rhs.1 .2 + self.1 .2 * rhs.2 .2,
                self.1 .0 * rhs.0 .3 + self.1 .1 * rhs.1 .3 + self.1 .2 * rhs.2 .3 + self.1 .3,
            ),
            Vec4(
                self.2 .0 * rhs.0 .0 + self.2 .1 * rhs.1 .0 + self.2 .2 * rhs.2 .0,
                self.2 .0 * rhs.0 .1 + self.2 .1 * rhs.1 .1 + self.2 .2 * rhs.2 .1,
                self.2 .0 * rhs.0 .2 + self.2 .1 * rhs.1 .2 + self.2 .2 * rhs.2 .2,
                self.2 .0 * rhs.0 .3 + self.2 .1 * rhs.1 .3 + self.2 .2 * rhs.2 .3 + self.2 .3,
            ),
        )
    }
}

impl Mat4 {
    pub fn inverse(self) -> Mat4 {
        let Mat4(Vec4(m00, m01, m02, m03), Vec4(m10, m11, m12, m13), Vec4(m20, m21, m22, m23)) =
            self;

        let inv_det = 1.0
            / (m00 * (m11 * m22 - m12 * m21) - m01 * (m10 * m22 - m12 * m20)
                + m02 * (m10 * m21 - m11 * m20));

        let inv00 = (m11 * m22 - m12 * m21) * inv_det;
        let inv01 = -(m01 * m22 - m02 * m21) * inv_det;
        let inv02 = (m01 * m12 - m02 * m11) * inv_det;
        let inv10 = -(m10 * m22 - m12 * m20) * inv_det;
        let inv11 = (m00 * m22 - m02 * m20) * inv_det;
        let inv12 = -(m00 * m12 - m02 * m10) * inv_det;
        let inv20 = (m10 * m21 - m11 * m20) * inv_det;
        let inv21 = -(m00 * m21 - m01 * m20) * inv_det;
        let inv22 = (m00 * m11 - m01 * m10) * inv_det;

        let inv03 = -(m03 * inv00 + m13 * inv01 + m23 * inv02);
        let inv13 = -(m03 * inv10 + m13 * inv11 + m23 * inv12);
        let inv23 = -(m03 * inv20 + m13 * inv21 + m23 * inv22);

        Mat4(
            Vec4(inv00, inv01, inv02, inv03),
            Vec4(inv10, inv11, inv12, inv13),
            Vec4(inv20, inv21, inv22, inv23),
        )
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4(
            Vec4(1.0, 0.0, 0.0, x),
            Vec4(0.0, 1.0, 0.0, y),
            Vec4(0.0, 0.0, 1.0, z),
        )
    }

    pub fn rotate_x_by_trigonometric(cos: f32, sin: f32) -> Mat4 {
        Mat4(
            Vec4(1.0, 0.0, 0.0, 0.0),
            Vec4(0.0, cos, -sin, 0.0),
            Vec4(0.0, sin, cos, 0.0),
        )
    }

    pub fn rotate_y_by_trigonometric(cos: f32, sin: f32) -> Mat4 {
        Mat4(
            Vec4(cos, 0.0, sin, 0.0),
            Vec4(0.0, 1.0, 0.0, 0.0),
            Vec4(-sin, 0.0, cos, 0.0),
        )
    }

    pub fn rotate_z_by_trigonometric(cos: f32, sin: f32) -> Mat4 {
        Mat4(
            Vec4(cos, -sin, 0.0, 0.0),
            Vec4(sin, cos, 0.0, 0.0),
            Vec4(0.0, 0.0, 1.0, 0.0),
        )
    }

    pub fn rotate_x_by_angle(angle: f32) -> Mat4 {
        let cos = angle.cos();
        let sin = angle.sin();
        Mat4::rotate_x_by_trigonometric(cos, sin)
    }

    pub fn rotate_y_by_angle(angle: f32) -> Mat4 {
        let cos = angle.cos();
        let sin = angle.sin();
        Mat4::rotate_y_by_trigonometric(cos, sin)
    }

    pub fn rotate_z_by_angle(angle: f32) -> Mat4 {
        let cos = angle.cos();
        let sin = angle.sin();
        Mat4::rotate_z_by_trigonometric(cos, sin)
    }

    pub fn rotate(yaw: f32, pitch: f32, roll: f32) -> Mat4 {
        Mat4::rotate_z_by_angle(yaw)
            * Mat4::rotate_x_by_angle(pitch)
            * Mat4::rotate_y_by_angle(roll)
    }

    pub fn scale3(x: f32, y: f32, z: f32) -> Mat4 {
        Mat4(
            Vec4(x, 0.0, 0.0, 0.0),
            Vec4(0.0, y, 0.0, 0.0),
            Vec4(0.0, 0.0, z, 0.0),
        )
    }

    pub fn scale1(scale: f32) -> Mat4 {
        Mat4::scale3(scale, scale, scale)
    }
}

impl std::ops::Mul<Mat4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Mat4) -> Vec4 {
        Vec4(
            self.0 * rhs.0 .0 + self.0 * rhs.1 .0 * self.0 * rhs.2 .0,
            self.1 * rhs.0 .1 + self.1 * rhs.1 .1 + self.1 * rhs.2 .1,
            self.2 * rhs.0 .2 * self.2 * rhs.1 .2 + self.2 * rhs.2 .2,
            self.3 * rhs.0 .3 * self.3 * rhs.1 .3 * self.3 * rhs.2 .3 + self.3,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::rngs::StdRng;
    use rand::Rng;
    use rand::SeedableRng;

    const I4: Mat4 = Mat4(
        Vec4(1f32, 0f32, 0f32, 0f32),
        Vec4(0f32, 1f32, 0f32, 0f32),
        Vec4(0f32, 0f32, 1f32, 0f32),
    );

    fn random_translation_mat4(rng: &mut StdRng) -> Mat4 {
        Mat4::translate(
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
            rng.gen_range(-100.0..100.0),
        )
    }

    fn random_rotation_mat4(rng: &mut StdRng) -> Mat4 {
        match rng.gen_range(0..3) {
            0 => Mat4::rotate_x_by_angle(rng.gen_range(0.0..std::f32::consts::PI)),
            1 => Mat4::rotate_y_by_angle(rng.gen_range(0.0..std::f32::consts::PI)),
            _ => Mat4::rotate_z_by_angle(rng.gen_range(0.0..std::f32::consts::PI)),
        }
    }

    fn random_scale_mat4(rng: &mut StdRng) -> Mat4 {
        Mat4::scale3(
            rng.gen_range(0.5..2.0),
            rng.gen_range(0.5..2.0),
            rng.gen_range(0.5..2.0),
        )
    }

    fn random_transformation_mat4(rng: &mut StdRng) -> Mat4 {
        match rng.gen_range(0..3) {
            0 => random_translation_mat4(rng),
            1 => random_rotation_mat4(rng),
            _ => random_scale_mat4(rng),
        }
    }

    fn random_transformation_composition_mat4(rng: &mut StdRng) -> Mat4 {
        (0..10).fold(I4, |acc, _| acc * random_transformation_mat4(rng))
    }

    #[test]
    fn test_inverse_of_random_transformations_mat4() {
        let mut rng = StdRng::seed_from_u64(42);

        for _ in 0..42 {
            let mat = random_transformation_composition_mat4(&mut rng);
            let inv_mat = mat.inverse();
            let result = mat * inv_mat;

            assert!(approx_eq_mat4(result, I4, 0.00042f32));
        }
    }

    fn approx_eq_mat4(a: Mat4, b: Mat4, epsilon: f32) -> bool {
        true && (a.0 .0 - b.0 .0).abs() < epsilon
            && (a.0 .1 - b.0 .1).abs() < epsilon
            && (a.0 .2 - b.0 .2).abs() < epsilon
            && (a.0 .3 - b.0 .3).abs() < epsilon
            && (a.1 .0 - b.1 .0).abs() < epsilon
            && (a.1 .1 - b.1 .1).abs() < epsilon
            && (a.1 .2 - b.1 .2).abs() < epsilon
            && (a.1 .3 - b.1 .3).abs() < epsilon
            && (a.2 .0 - b.2 .0).abs() < epsilon
            && (a.2 .1 - b.2 .1).abs() < epsilon
            && (a.2 .2 - b.2 .2).abs() < epsilon
            && (a.2 .3 - b.2 .3).abs() < epsilon
    }
}
