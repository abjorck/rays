mod vec3;
mod ray;

use vec3::Color;

use std::cell::{RefCell, Ref};
use crate::vec3::{Point, Vec3};
use crate::ray::Ray;

fn main() {
    //println!("Hello, world!");
    const aspect_ratio: f32 = 16.0 / 9.0;
    const image_width: usize = 800;
    const image_height: usize = (image_width as f32 / aspect_ratio) as usize;
    const w: usize = image_width;
    const h: usize = image_height;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point { x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3 { x: 0.0, y: 0.0, z: focal_length };


    let mut p = vec![Pixel::new(0.0, 0.0, 0.0); w * h];

    for j in (0..h).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..w {
            //p[i+w*j] = Pixel::new((i as f32)/(w as f32), j as f32/h as f32, 0.25);
            let u = (i as f32) / ((image_width - 1) as f32);
            let v = (j as f32) / ((image_height - 1) as f32);
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            p[i + w * j] = ray_color(r);
        }
    }

    let i = Image {
        w: w as u32,
        h: h as u32,
        pixels: p,
    };
    i.print_ppm();
}

fn ray_color(r: Ray) -> Color {
    let t =  hit_sphere(Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    }, 0.5, &r);
    if t > 0.0 {
        let n: Vec3 = r.at(t).unit_vector() - Vec3::newi(0,0,-1);
        return Color(0.5 * Vec3::new(n.x+1.0, n.y+1.0, n.z+1.0));
    }
    let unit_direction = r.dir.unit_vector();
    let len = r.dir.length_squared();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Color((1.0 - t) * Color::new(1.0, 1.0, 1.0).0 + t * Color::new(0.5, 0.7, 1.0).0);
}

fn hit_sphere(center: Point, radius: f32, r: &Ray) -> f32 {
    let oc = r.orig - center;
    let a = r.dir.dot(&r.dir);
    let b = 2.0 * oc.dot(&r.dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

type Pixel = Color;

struct Image {
    w: u32,
    h: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    fn print_ppm(&self) {
        println!("P3");
        println!("{} {}", self.w, self.h);
        println!("255");
        for row in self.pixels.chunks(self.w as usize).rev() {
            for p in row {
                print!("{} ", p);
            }
            println!();
        }
    }
}
