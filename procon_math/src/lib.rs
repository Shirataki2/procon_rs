extern crate __procon_math_traits as math_traits;

use math_traits::PrimitiveInteger as Int;

pub fn gcd<T: Int>(a: T, b: T) -> T {
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<T: Int>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

pub fn extgcd<T: Int>(a: T, b: T) -> (T, T, T) {
    if b > T::zero() {
        let (g, mut y, x) = extgcd(b, a % b);
        y -= (a / b) * x;
        (g, x, y)
    } else {
        (a, T::one(), T::zero())
    }
}

pub fn powmod<T: Int>(mut x: T, mut n: T, modulo: T) -> T {
    let mut ret = T::one();
    while n > T::zero() {
        if n & T::one() != T::zero() {
            ret = (ret * x) % modulo;
        }
        x = (x * x) % modulo;
        n >>= T::one();
    }
    ret
}

pub fn invmod<T: Int>(x: T, modulo: T) -> T {
    let (_d, x, _y) = extgcd(x, modulo);
    signed_mod(x, modulo)
}

pub fn signed_mod<T: Int>(x: T, modulo: T) -> T {
    (x % modulo + modulo) % modulo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_small() {
        assert_eq!(gcd(12, 15), 3);
        assert_eq!(gcd(60, 75), 15);
        assert_eq!(gcd(19, 48), 1);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(1, 5), 1);
    }

    #[test]
    fn test_lcm_small() {
        assert_eq!(lcm(12, 15), 60);
        assert_eq!(lcm(60, 75), 300);
        assert_eq!(lcm(19, 48), 912);
        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(1, 5), 5);
    }

    #[test]
    fn test_gcd_large() {
        assert_eq!(gcd(4785420, 4478120), 20);
        assert_eq!(gcd(187812024_i64, 563436072), 187812024);
    }

    #[test]
    fn test_lcm_large() {
        assert_eq!(lcm(4785420_i64, 4478120), 1071484250520);
        assert_eq!(lcm(187812024_i64, 563436072), 563436072);
    }

    #[test]
    fn test_extgcd() {
        assert_eq!(extgcd(12, 15), (3, -1, 1));
        assert_eq!(extgcd(78947, 67465), (1, 29643, -34688));
        assert_eq!(extgcd(47_i64, 998244353), (1, 191153174, -9));
    }

    #[test]
    fn test_signed_mod() {
        assert_eq!(signed_mod(-1, 7), 6);
        assert_eq!(signed_mod(-14, 7), 0);
        assert_eq!(signed_mod(-100, 7), 5);
        assert_eq!(signed_mod(4, 7), 4);
        assert_eq!(signed_mod(9, 7), 2);
    }

    #[test]
    fn test_invmod() {
        assert_eq!(invmod(3, 7), 5);
        assert_eq!(invmod(2, 429), 215);
        assert_eq!(invmod(123_456_789_i64, 1_000_000_007), 18_633_540);
    }

    #[test]
    fn test_powmod_mod7() {
        assert_eq!(powmod(2, 2, 7), 4);
        assert_eq!(powmod(2, 3, 7), 1);
        assert_eq!(powmod(2, 0, 7), 1);
    }

    #[test]
    fn test_powmod_mod1e9p7() {
        let m = 1_000_000_007_i64;
        assert_eq!(powmod(18, 75, m), 879190096);
        assert_eq!(powmod(977812, 8877774, m), 758213842);
    }
}
