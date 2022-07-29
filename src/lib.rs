#![feature(trait_alias)]
#![feature(associated_type_bounds)]
// #![feature(adt_const_params)]

extern crate core;

pub mod structures;
pub mod implementations;
pub mod power;
pub mod matrix;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
