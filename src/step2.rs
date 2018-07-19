#[derive(Debug)]
struct VStruct<T>(Vec<T>);

trait VSGeneric<T> {
    fn new_vsg() -> VStruct<T>;
}

impl VSGeneric<u8> for VStruct<u8> {
    fn new_vsg() -> VStruct<u8> {
        VStruct(vec![8])
    }
}

impl VSGeneric<u16> for VStruct<u16> {
    fn new_vsg() -> VStruct<u16> {
        VStruct(vec![16])
    }
}

pub fn step2() {
    let vsgu8 = <VStruct<u8> as VSGeneric<u8>>::new_vsg();
    let vsgu16 = <VStruct<u16> as VSGeneric<u16>>::new_vsg();
    println!("VSGeneric<u8>: {:?}", vsgu8);
    println!("VSGeneric<u16>: {:?}", vsgu16);
}
