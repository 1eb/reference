extern crate bmp;

use std::io::Error;

use bmp::{Image, Pixel};
use project_1eb_reference_core::base_types::{HdrColor, LdrColor, Position, Transform};
use project_1eb_reference_core::scene::{Material, PerspectiveCamera, Plane, Scene, F0_GOLD};
use project_1eb_reference_core::{render_hdr_image, render_ldr_image};

const GAMMA: f32 = 2.2f32;
const EXPOSURE: f32 = 1.0f32;
const SQRT_SUPER_SAMPLING_RATE: usize = 1;
const WIDTH: usize = 900;
const HEIGHT: usize = 540;

fn main() -> Result<(), Error> {
    let scene = Scene::new(
        Box::new(PerspectiveCamera::by_x(
            Position::new(0f32, 0f32, 0f32),
            0f32,
            0f32,
            0f32,
            WIDTH as f32 / HEIGHT as f32,
            1f32,
        )),
        Box::new(Plane::new(
            Transform::I,
            1f32,
            0f32,
            0f32,
            1f32,
            Material {
                albedo: LdrColor {
                    r: 1f32,
                    g: 1f32,
                    b: 0.2f32,
                },
                roughness: 0.42f32,
                f0: F0_GOLD,
            },
        )),
        HdrColor {
            r: 1f32,
            g: 1f32,
            b: 1f32,
        },
        vec![],
    );

    let output = render_ldr_image(
        render_hdr_image(&scene, WIDTH, HEIGHT),
        EXPOSURE,
        GAMMA,
        SQRT_SUPER_SAMPLING_RATE,
        LdrColor {
            r: 0f32,
            g: 0f32,
            b: 0f32,
        },
    );

    let mut img = Image::new(output.width as u32, output.height as u32);
    for (x, y) in img.coordinates() {
        let pixel = output.content[y as usize * output.width + x as usize];
        let r = (pixel.r * 255.999f32).floor() as u8;
        let g = (pixel.g * 255.999f32).floor() as u8;
        let b = (pixel.b * 255.999f32).floor() as u8;
        img.set_pixel(x, y, Pixel::new(r, g, b));
    }
    img.save("img.bmp")
}
