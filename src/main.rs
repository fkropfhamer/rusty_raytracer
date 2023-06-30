use std::fs;

use crate::{hittable_list::HittableList, camera::Camera};
use rand::Rng;

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod camera;

    
fn main(){
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 100;
    let scale = 1.0 / (samples_per_pixel as f64);
    let max_depth = 50;

    //World
    let world = HittableList { 
        objects: vec![Box::new(sphere::new(vec3::new(0.0, -100.5, -1.0), 100.0)), Box::new(sphere::new(vec3::new(0.0, 0.0, -1.0), 0.5))]
    };

    //Camera
    let camera = Camera::new(); 

    let mut img_str = "".to_owned();
    img_str.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));

    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = vec3::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width-1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height-1) as f64;
                
                let ray = camera.get_ray(u, v);

                pixel_color = pixel_color + ray.get_color(&world, max_depth);
            }
            
            img_str.push_str(&pixel_color.to_color_string(scale))
        }
    }
    
    fs::write("image.ppm", img_str).expect("Unable to write file");
    println!("\nDone!");
}
