use myraytracing::Vec3;

fn main() {
    let a = Vec3::new(0.1, 0.2, 0.3);
    let unit_direction = a.unit_vector();

    println!("{}", unit_direction);
}
