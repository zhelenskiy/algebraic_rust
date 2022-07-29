#![feature(trait_alias)]
#![feature(associated_type_bounds)]

mod structures;
mod implementations;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
