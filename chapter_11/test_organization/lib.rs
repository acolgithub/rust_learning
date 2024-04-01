pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// public function that adds two
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// private adder function
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;  // brings parent's items into scope of child module

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // test private function
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
