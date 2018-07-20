use std::fmt::Debug;

#[derive(Debug)]
struct VStruct<T>(Vec<T>);

trait VSAssoc {
    fn new_vsa() -> Self;
}

impl VSAssoc for VStruct<String> {
    fn new_vsa() -> VStruct<String> {
        VStruct(vec!["string".to_string()])
    }
}

impl VSAssoc for VStruct<u8> {
    fn new_vsa() -> VStruct<u8> {
        VStruct(vec![8])
    }
}

fn x<V: VSAssoc + Debug>() {
    println!("{:?}", V::new_vsa())
}

pub fn step3() {
    let vsas = <VStruct<String> as VSAssoc>::new_vsa();
    let vsau8 = <VStruct<u8> as VSAssoc>::new_vsa();
    println!("SVAssoc<String>: {:?}", vsas);
    println!("SVAssoc<u8>: {:?}", vsau8);
    x::<VStruct<String>>()
}
