pub mod integers;
mod helpers;
pub mod fractionals;
pub mod matrix;
pub mod finite_matrix;


#[cfg(test)]
mod tests {
    use num::{BigInt, BigUint, Complex};
    use num::rational::Ratio;
    use crate::structures::ring_like::RingWithOne;
    use crate::structures::ring_like::SemiringWithOne;
    use crate::structures::ring_like::Field;

    fn supplier_ring<T: RingWithOne>() {}

    fn supplier_semiring<T: SemiringWithOne>() {}

    fn supplier_field<T: Field>() {}

    #[test]
    fn it_works() {
        supplier_semiring::<u8>();
        supplier_semiring::<u16>();
        supplier_semiring::<u32>();
        supplier_semiring::<u64>();
        #[cfg(has_i128)]
        supplier_semiring::<u128>();
        supplier_semiring::<usize>();
        supplier_semiring::<BigUint>();
        supplier_semiring::<Ratio<u8>>();
        supplier_semiring::<Complex<u8>>();

        supplier_ring::<i8>();
        supplier_ring::<i16>();
        supplier_ring::<i32>();
        supplier_ring::<i64>();
        #[cfg(has_i128)]
        supplier_ring::<i128>();
        supplier_ring::<isize>();
        supplier_ring::<BigInt>();
        supplier_ring::<Ratio<i8>>();
        supplier_ring::<Complex<i8>>();

        supplier_field::<f32>();
        supplier_field::<f64>();
        supplier_field::<Ratio<i32>>();
        supplier_field::<Complex<f32>>();
    }
}