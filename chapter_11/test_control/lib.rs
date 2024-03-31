pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// function that prints parameter and returns 10s
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);  // when we test function we will not see this print if test passes since output is captured
    10                                  // only failure allows print
}                                       // to see output when passes use "-- --show-output" flag

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // test that should work
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    // test that will not work
    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);  // fails since return value of function is 10
    }

    // run single tests by using name of function (i.e. cargo test FUNCTION_NAME)
    // run multiple tests using regex (i.e. specify part of name to match like "cargo test add")

    // first test on add_two function
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    // second test on add_two function
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    // third test on add_two function
    #[test]
    fn one_hundre() {
        assert_eq!(102, add_two(100));
    }

    // can use ignore attribute to exclude time-consuming tests
    // if we only want ignored tests use "cargo test -- --ignored"
    // we can run everything by using "cargo test -- --include-ignored"
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
