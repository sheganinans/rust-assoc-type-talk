#[derive(Debug)]
struct VStruct<T>(Vec<T>);

trait VSAssoc {
    type T;
    fn new_vsa(&self) -> VStruct<Self::T>;
}

impl VSAssoc for VStruct<String> {
    type T = String;
    fn new_vsa(&self) -> VStruct<String> {
        VStruct(vec!["string".to_string()])
    }
}

impl VSAssoc for VStruct<u8> {
    type T = u8;
    fn new_vsa(&self) -> VStruct<u8> {
        VStruct(vec![8])
    }
}

pub fn step4() {
    let vsinst = VStruct(vec!["string".to_string()]);
    let newvsa = vsinst.new_vsa();
    //let vsas = <VStruct<String> as VSAssoc>::new_vsa();
    //let vsau8 = <VStruct<u8> as VSAssoc>::new_vsa();
    println!("SVAssoc<String>: {:?}", newvsa);
    //println!("SVAssoc<u8>: {:?}", vsau8)
}
