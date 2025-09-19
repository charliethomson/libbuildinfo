fn main() {
    let bi = libbuildinfo::load_build_info!();

    println!("{:#?}", bi);
}
