#![no_std]

use abstra::{BoolSlice, Set, Slice};

#[cfg(test)]
mod tests {
    #[test]
    fn no_std_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
