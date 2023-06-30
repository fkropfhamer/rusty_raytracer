use std::{thread, fs, sync::{mpsc::channel, Arc}, cmp::Ordering};

use crate::{hittable_list::HittableList, camera::Camera};
use rand::Rng;
use threadpool::ThreadPool;

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
        objects: vec![Arc::new(sphere::new(vec3::new(0.0, -100.5, -1.0), 100.0)), Arc::new(sphere::new(vec3::new(0.0, 0.0, -1.0), 0.5))]
    };

    //Camera
    let camera = Camera::new(); 

    let mut img_str = "".to_owned();
    img_str.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));


    let n_workers = 22;
    let n_jobs = image_width * image_height;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel::<PixResult>();
    let (ctx, crx) = channel();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let tx = tx.clone();
            let ctx = ctx.clone();

            let c = camera.clone();
            let w = world.clone();
                
            pool.execute(move|| {
                let index = (image_height - j) * image_width + i;
                let pixel_color = cal_pix(c, w, samples_per_pixel, i, j, max_depth, image_width, image_height);

                let r = PixResult {
                    pixel_color,
                    index, 
                };

                ctx.send(1).expect("c channel error");
                tx.send(r).expect("channel error");
            });
        }
    }

    thread::spawn(move|| {
        let mut c = 0;
        
        for _ in crx.iter() {
            c += 1;
            print!("\r {} of {}", c, n_jobs); 
        }
    });

    let mut rs = rx.iter().take(n_jobs as usize).collect::<Vec<PixResult>>();
    rs.sort();

    for r in rs.iter() {
        img_str.push_str(&r.pixel_color.to_color_string(scale))
    }

    fs::write("image.ppm", img_str).expect("Unable to write file");
    println!("\nDone!");
}

fn cal_pix(camera: camera::Camera, world: HittableList, samples_per_pixel: i64, i: i64, j: i64, max_depth: i64, image_width: i64, image_height: i64) -> vec3::Vec3 {
    let mut rng = rand::thread_rng();
    let mut pixel_color = vec3::new(0.0, 0.0, 0.0);

    for _ in 0..samples_per_pixel {
        let u = (i as f64 + rng.gen::<f64>()) / (image_width-1) as f64;
        let v = (j as f64 + rng.gen::<f64>()) / (image_height-1) as f64;
                
        let ray = camera.get_ray(u, v);

        pixel_color = pixel_color + ray.get_color(&world, max_depth);
    }

    pixel_color
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct PixResult {
    index: i64,
    pixel_color: vec3::Vec3,
}

impl Ord for PixResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

