fn main() {
    let bi = libbuildinfo::load_build_info!(optional);

    println!("{:#?}", bi);
}
