trait Animal {
    fn num_legs(&self) -> usize {
        4
    }
}

struct Dog;
impl Animal for Dog {
    // use the default impl
}

struct Chicken;
impl Animal for Chicken {
    fn num_legs(&self) -> usize {
        2
    }
}

fn print_num_legs(animal: &Animal) {
    println!("legs: {}", animal.num_legs());
}

pub fn step1() {
    let dog = Dog;
    let chicken = Chicken;

    print_num_legs(&dog);
    print_num_legs(&chicken);
}
