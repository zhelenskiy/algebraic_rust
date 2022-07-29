
pub mod unsigned {
    use crate::structures::ring_like::{RingWithOne, one, multi};

    pub fn pow<T: RingWithOne + Clone>(base: T, exponent: u64) -> T {
        if exponent == 0 {
            one()
        } else if exponent % 2 == 0 {
            let t = pow(base, exponent / 2);
            multi(t.clone(), t)
        } else {
            multi(pow(base.clone(), exponent - 1), base)
        }
    }
}

pub mod signed {
    use crate::structures::ring_like::{Field, reciprocal};
    use super::unsigned::pow as u_pow;

    pub fn pow<T: Field + Clone>(base: T, exponent: i64) -> T {
        if exponent == i64::MIN {
            u_pow(pow(base, exponent / 2), 2)
        } else if exponent < 0 {
            reciprocal(pow(base, -exponent))
        } else {
            u_pow(base, exponent as u64)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{signed, unsigned};

    #[test]
    fn unsigned() {
        assert_eq!(unsigned::pow(2, 10), 1024);
        assert_eq!(unsigned::pow(2, 0), 1);
        assert_eq!(signed::pow(2.0, 10), 1024.0);
        assert_eq!(signed::pow(2.0, 0), 1.0);
        assert_eq!(signed::pow(2.0, -1), 0.5);
    }
}