mod vec3;

fn main() {
    let vec1 = vec3::init(1.0, 2.0, 3.0);
    let vec2 = vec3::init(1.0, 2.0, 3.0);

    let _vec3 = vec3::add(vec1, vec2);

    println!("Hello, world!");
}
