use myraytracing::Vec3;

fn main() {
    let a = Vec3::new(0.1, 0.2, 0.3);
    let b = Vec3::new(0.4, 0.5, 0.6);

    println!("-a = {}", -a);
    println!("a + b = {}", a + b);
    println!("a * b = {}", a * b);
}
