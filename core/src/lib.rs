use base_types::{Direction, HdrColor, LdrColor};
use scene::{Intersection, Material, Scene};

pub mod base_types;
pub mod math;
pub mod scene;

pub const EPSILON: f32 = 0.00042f32;

pub fn render(scene: &Scene, position_in_image: (f32, f32)) -> Option<HdrColor> {
    let Intersection {
        position,
        real_normal,
        adjusted_normal,
        material,
    } = scene
        .world
        .intersect(&scene.camera.get_ray(position_in_image))?;
    let adjusted_position = position + real_normal * EPSILON;

    let Material {
        albedo,
        roughness,
        f0,
    } = material;
    Some(
        scene
            .lights
            .iter()
            .fold(albedo * scene.ambient_light, |acc, curr| {
                if let Some((color, direction)) = curr.illuminate(adjusted_position, &*scene.world)
                {
                    acc + albedo
                        * color
                        * brdf(
                            Direction::from_movement(scene.camera.position() - position),
                            -direction,
                            adjusted_normal,
                            roughness,
                            f0,
                        )
                } else {
                    acc
                }
            }),
    )
}

fn brdf(
    surface_to_view: Direction,
    surface_to_light: Direction,
    surface_normal: Direction,
    roughness: f32,
    f0: f32,
) -> f32 {
    fn fresnel_schlick(cos_theta: f32, f0: f32) -> f32 {
        f0 + (1f32 - f0) * (1f32 - cos_theta).powf(5f32)
    }

    fn ggx_ndf(n: Direction, h: Direction, roughness: f32) -> f32 {
        let alpha = roughness * roughness;
        let alpha2 = alpha * alpha;
        let cos_n_h = n.cos_angle_between(h);
        let cos_n_h2 = cos_n_h * cos_n_h;
        let denom = cos_n_h2 * alpha2 + (1f32 - cos_n_h2);
        alpha2 / (std::f32::consts::PI * denom * denom)
    }

    fn geometric_attenuation(n: Direction, v: Direction, l: Direction, roughness: f32) -> f32 {
        let k = (roughness + 1f32) * (roughness + 1f32) / 8f32;
        let cos_n_v = n.cos_angle_between(v);
        let g_v = cos_n_v / (cos_n_v * (1f32 - k) + k);
        let cos_n_l = n.cos_angle_between(l);
        let g_l = cos_n_l / (cos_n_l * (1f32 - k) + k);
        g_v * g_l
    }

    fn cook_torrance_specular(
        v: Direction,
        l: Direction,
        n: Direction,
        roughness: f32,
        f0: f32,
    ) -> f32 {
        let h = Direction::from_directions([v, l]);
        let d = ggx_ndf(n, h, roughness);
        let f = fresnel_schlick(h.cos_angle_between(v), f0);
        let g = geometric_attenuation(n, v, l, roughness);
        let specular = (d * f * g) / (4f32 * n.cos_angle_between(v) * n.cos_angle_between(v));
        specular
    }

    cook_torrance_specular(
        surface_to_view,
        surface_to_light,
        surface_normal,
        roughness,
        f0,
    )
}

pub struct HdrImage {
    pub width: usize,
    pub height: usize,
    pub content: Vec<Option<HdrColor>>,
}

pub fn render_hdr_image(scene: &Scene, width: usize, height: usize) -> HdrImage {
    let mut content = Vec::new();
    for y in 0..height {
        for x in 0..width {
            content.push(render(
                scene,
                (
                    x as f32 / (width - 1) as f32,
                    1.0f32 - y as f32 / (height - 1) as f32,
                ),
            ))
        }
    }
    HdrImage {
        width,
        height,
        content,
    }
}

pub struct LdrImage {
    pub width: usize,
    pub height: usize,
    pub content: Vec<LdrColor>,
}

fn resize(image: &LdrImage, sqrt_super_sampling_rate: usize) -> LdrImage {
    let white = HdrColor::new(1.0f32, 1.0f32, 1.0f32);
    let width = image.width / sqrt_super_sampling_rate;
    let height = image.height / sqrt_super_sampling_rate;
    let mut content = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let mut current = Vec::new();
            for i in 0..sqrt_super_sampling_rate {
                for j in 0..sqrt_super_sampling_rate {
                    current.push(
                        image.content[(y * sqrt_super_sampling_rate + i) * image.width
                            + x * sqrt_super_sampling_rate
                            + j],
                    )
                }
            }
            let HdrColor { r, g, b } =
                current
                    .iter()
                    .fold(HdrColor::new(0f32, 0f32, 0f32), |acc, curr| {
                        acc + *curr
                            * (sqrt_super_sampling_rate * sqrt_super_sampling_rate) as f32
                            * white
                    });
            content.push(LdrColor::new(r, g, b))
        }
    }
    LdrImage {
        width,
        height,
        content,
    }
}

pub fn render_ldr_image(
    hdr_image: HdrImage,
    exposure: f32,
    gamma: f32,
    sqrt_super_sampling_rate: usize,
    void_color: LdrColor,
) -> LdrImage {
    let HdrImage {
        width,
        height,
        content,
    } = hdr_image;

    resize(
        &LdrImage {
            width: width * sqrt_super_sampling_rate,
            height: height * sqrt_super_sampling_rate,
            content: content
                .iter()
                .map(|&color| {
                    if let Some(HdrColor { r, g, b }) = color {
                        let r = (1f32 - (-r * exposure).exp()).powf(1f32 / gamma);
                        let g = (1f32 - (-g * exposure).exp()).powf(1f32 / gamma);
                        let b = (1f32 - (-b * exposure).exp()).powf(1f32 / gamma);
                        LdrColor { r, g, b }
                    } else {
                        void_color
                    }
                })
                .collect(),
        },
        sqrt_super_sampling_rate,
    )
}
