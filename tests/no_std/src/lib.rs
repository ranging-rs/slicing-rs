#![no_std]

use ranging::{BoolSlice, Set, Slice};
// TODO test that the following fails to compile
//use ranging::hash::Set;

#[cfg(test)]
mod tests {
    #[test]
    fn no_std_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
