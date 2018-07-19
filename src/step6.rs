#[derive(Debug)]
struct VStruct<T, U>(Vec<T>, Vec<U>);

trait VSAssoc {
    fn new_vsa() -> Self;
}

impl VSAssoc for VStruct<String, ()> {
    fn new_vsa() -> VStruct<String, ()> {
        VStruct(vec!["string".to_string()], vec![()])
    }
}

trait VSGeneric<T> {
    fn new_vsg() -> Self;
}

impl VSGeneric<()> for VStruct<String, ()> {
    fn new_vsg() -> VStruct<String, ()> {
        VStruct(vec!["vsgeneric ()".to_string()], vec![()])
    }
}

impl VSGeneric<u8> for VStruct<String, ()> {
    fn new_vsg() -> VStruct<String, ()> {
        VStruct(vec!["vsgeneric u8".to_string()], vec![()])
    }
}

trait VSGeneric2<T, U> {
    fn new_vsg() -> VStruct<T, U>;
}

impl VSGeneric2<String, ()> for VStruct<String, ()> {
    fn new_vsg() -> VStruct<String, ()> {
        VStruct(vec!["string".to_string()], vec![()])
    }
}

/*
impl VSGeneric2<String, u8> for VStruct<String, ()> {
    fn new_vsg() -> VStruct<String, ()> {
        VStruct(vec!["string".to_string()], vec![()])
    }
}
*/

pub fn step6() {
    let vsas = <VStruct<String, ()> as VSAssoc>::new_vsa();
    let vsgunit = <VStruct<String, ()> as VSGeneric<()>>::new_vsg();
    let vsgu8 = <VStruct<String, ()> as VSGeneric<u8>>::new_vsg();
    println!("SVAssoc<String>: {:?}", vsas);
    println!("SVGeneric<()>: {:?}", vsgunit);
    println!("SVGeneric<u8>: {:?}", vsgu8);
    let vsgstrunit = <VStruct<String, ()> as VSGeneric2<String, ()>>::new_vsg();
    println!("SVGeneric2<String, ()>: {:?}", vsgstrunit)
}
