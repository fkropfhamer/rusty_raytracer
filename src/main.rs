use std::fs;


mod vec3;

fn main(){
    let image_width = 256;
    let image_height = 256;

    let mut img_str = "".to_owned();

    println!("P3\n{} {}\n255\n", image_width, image_height);
    img_str.push_str(&format!("P3\n{} {}\n255\n", image_width, image_height));


    for j in (0..image_height).rev() {
        //println!("j: {}", j);
        for i in 0..image_width {
            //println!("i: {}", i);

            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {} \n", ir, ig, ib);
            img_str.push_str(&format!("{} {} {} \n", ir, ig, ib));
        }
    }

    fs::write("image.ppm", img_str).expect("Unable to write file");
}
