use metarust_demo::*;

fn main() {
    let v = Vec4::new(1., 2., 3., 4.);

    println!("αβγδ: {:?}", v);
    println!("δγβα: {:?}", v.δγβα());
}
