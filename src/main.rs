struct Qubit {
    is_active: bool,
}

impl Zero for Qubit {
    fn is_zero(&self) {
        println!("State: is 0 -> {}", self.is_active);
    }
}

impl One for Qubit {
    fn is_one(&self) -> () {
        println!("State: is 1 -> {}", self.is_active);
    }
}

trait Zero {
    fn is_zero(&self) -> ();
}

trait One {
    fn is_one(&self) -> ();
}

trait SuperPosition: Zero + One {}

impl SuperPosition for Qubit {}

fn compute(bit: &dyn SuperPosition) -> () {
    bit.is_zero();
    bit.is_one();
}

fn main() {
    let qubit: Qubit = Qubit { is_active: true };
    compute(&qubit);
}
