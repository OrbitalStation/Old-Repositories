use vecs::*;

fn main() {
    let v1 = vec3::new(1., 2., 3.);
    println!("vec3::new(1., 2., 3.)                = {:?}", v1);

    let v2 = vec4::new();
    println!("vec4::new()                          = {:?}", v2);

    let v3 = vec4::new(v1, 4.);
    println!("vec4::new(v1, 4.)                    = {:?}", v3);

    let v4 = bvec1::new(true);
    println!("bvec1::new(true)                     = {:?}", v4);

    let v5 = bvec4::new(v4, false, v4);
    println!("bvec4::new(v4, false, v4)            = {:?}", v5);

    let v6 = uvec3::new((2u32, 2u32), (), [1u32]);
    println!("uvec3::new((2u32, 2u32), (), [1u32]) = {:?}", v6);
}
