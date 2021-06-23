use std::fs;

mod vec3;
mod ray;
mod hittable;
mod sphere;

fn main(){
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - vec3::scale(0.5, horizontal) - vec3::scale(0.5, vertical) - vec3::new(0.0, 0.0, focal_length);

    let mut img_str = "".to_owned();
    img_str.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));


    for j in (0..image_height).rev() {
        print!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            
            let r = ray::new(origin, lower_left_corner + vec3::scale(u, horizontal) + vec3::scale(v, vertical) - origin);

            let pixel_color = ray::ray_color(&r);

            img_str.push_str(&pixel_color.to_color_string());
        }
    }
    
    fs::write("image.ppm", img_str).expect("Unable to write file");
    println!("\nDone!");
}
