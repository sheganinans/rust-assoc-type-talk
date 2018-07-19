#[derive(Debug)]
struct VStruct<T>(Vec<T>);

trait VSAssoc {
    type T;
    fn new_vsa() -> VStruct<Self::T>;
    fn first(&mut self) -> Option<Self::T>;
}

impl VSAssoc for VStruct<String> {
    type T = String;
    fn new_vsa() -> VStruct<String> {
        VStruct(vec!["string".to_string()])
    }

    fn first(&mut self) -> Option<Self::T> {
        self.0.pop()
    }
}

pub fn step5() {
    let mut vsas = <VStruct<String> as VSAssoc>::new_vsa();
    println!("SVAssoc<String>: {:?}", vsas);
    println!("Option<String>: {:?}", vsas.first())
}
