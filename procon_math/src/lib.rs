extern crate __procon_math_traits as math_traits;

use math_traits::PrimitiveInteger as Int;
use std::{hash, mem::swap};

pub fn divisor<T: Int>(n: T) -> Vec<T> {
    let mut ret = Vec::new();
    let mut i = T::one();
    loop {
        if i * i > n {
            break;
        }
        if n % i == T::zero() {
            ret.push(i);
            if i * i != n {
                ret.push(n / i);
            }
        }
        i += T::one();
    }
    ret.sort_unstable();
    ret
}

pub fn factorize<T: Int>(n: T) -> Vec<T> {
    let mut ret = vec![];
    let mut n = n;
    let two = T::one() + T::one();
    let three = T::one() + two;
    while n % two == T::zero() {
        ret.push(two);
        n /= two;
    }
    let mut i = three;
    while i * i <= n {
        while n % i == T::zero() {
            ret.push(i);
            n /= i;
        }
        i += two;
    }
    if n > two {
        ret.push(n)
    }
    ret
}

pub fn factorize_pair<T: Int + hash::Hash>(n: T) -> std::collections::HashMap<T, usize> {
    let mut ret = std::collections::HashMap::new();
    let mut n = n;
    let two = T::one() + T::one();
    let three = T::one() + two;

    while n % two == T::zero() {
        *ret.entry(two).or_insert(0) += 1;
        n /= two;
    }
    let mut i = three;
    while i * i <= n {
        while n % i == T::zero() {
            *ret.entry(i).or_insert(0) += 1;
            n /= i;
        }
        i += two;
    }
    if n > two {
        *ret.entry(n).or_insert(0) += 1;
    }
    ret
}

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

pub fn invgcd<T: Int>(a: T, b: T) -> (T, T) {
    let a = signed_mod(a, b);
    if a == T::zero() {
        return (b, T::zero());
    }
    // Contracts:
    // [1] s - m0 * a = 0 (mod b)
    // [2] t - m1 * a = 0 (mod b)
    // [3] s * |m1| + t * |m0| <= b
    let mut s = b;
    let mut t = a;
    let mut m0 = T::zero();
    let mut m1 = T::one();

    while t != T::zero() {
        let u = s / t;
        s -= t * u;
        m0 -= m1 * u; // |m1 * u| <= |m1| * s <= b

        // [3]:
        // (s - t * u) * |m1| + t * |m0 - m1 * u|
        // <= s * |m1| - t * u * |m1| + t * (|m0| + |m1| * u)
        // = s * |m1| + t * |m0| <= b

        swap(&mut s, &mut t);
        swap(&mut m0, &mut m1);
    }
    // by [3]: |m0| <= b/g
    // by g != b: |m0| < b/g
    if m0 < T::zero() {
        m0 += b / s;
    }
    (s, m0)
}

pub fn crt<T: Int>(r: &[T], m: &[T]) -> Option<(T, T)> {
    assert_eq!(r.len(), m.len());
    let (mut r0, mut m0) = (T::zero(), T::one());
    for (&(mut ri), &(mut mi)) in r.iter().zip(m.iter()) {
        assert!(mi >= T::one());
        if m0 < mi {
            swap(&mut r0, &mut ri);
            swap(&mut m0, &mut mi);
        }
        if m0 % mi == T::zero() {
            if r0 % mi != ri {
                return None;
            }
            continue;
        }
        let (g, im) = invgcd(m0, mi);
        let u1 = mi / g;
        if (ri - r0) % g != T::zero() {
            return None;
        }
        let x = (ri - r0) / g % u1 * im % u1;
        r0 += x * m0;
        m0 *= u1;
        if r0 < T::zero() {
            r0 += m0;
        }
    }
    Some((r0, m0))
}

pub fn floor_sum<T: Int>(n: T, m: T, mut a: T, mut b: T) -> T {
    let mut ans = T::zero();
    let two = T::one() + T::one();
    if a >= m {
        ans += (n - T::one()) * n * (a / m) / two;
        a %= m;
    }
    if b >= m {
        ans += n * (b / m);
        b %= m;
    }
    let y_max = (a * n + b) / m;
    let x_max = y_max * m - b;
    if y_max == T::zero() {
        return ans;
    }
    ans += (n - (x_max + a - T::one()) / a) * y_max;
    ans += floor_sum(y_max, a, m, (a - x_max % a) % a);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_divisor() {
        let s = divisor(108);
        assert_eq!(s, vec![1, 2, 3, 4, 6, 9, 12, 18, 27, 36, 54, 108]);
        let s = divisor(1);
        assert_eq!(s, vec![1]);
        let s = divisor(25);
        assert_eq!(s, vec![1, 5, 25]);
        let s = divisor(65536);
        assert_eq!(
            s,
            vec![
                1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536
            ]
        );
    }

    #[test]
    fn test_factorize() {
        assert_eq!(factorize(24), vec![2, 2, 2, 3]);
        assert_eq!(factorize(498640), vec![2, 2, 2, 2, 5, 23, 271]);
    }

    #[test]
    fn test_factorize_large_prime() {
        assert_eq!(factorize(1_000_000_000_039_i64), vec![1_000_000_000_039]);
    }

    #[test]
    fn test_factorize_pair() {
        let mut map = HashMap::new();
        map.insert(2, 4);
        map.insert(5, 1);
        map.insert(23, 1);
        map.insert(271, 1);
        assert_eq!(factorize_pair(498640), map);
    }

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

    #[test]
    fn test_crt() {
        let a = [44, 23, 13];
        let b = [13, 50, 22];
        assert_eq!(crt(&a, &b), Some((1773, 7150)));
        let a = [12345_i64, 67890, 99999];
        let b = [13, 444321, 95318];
        assert_eq!(crt(&a, &b), Some((103333581255, 550573258014)));
        let a = [0, 3, 4];
        let b = [1, 9, 5];
        assert_eq!(crt(&a, &b), Some((39, 45)));
    }

    #[test]
    fn test_floor_sum() {
        assert_eq!(floor_sum(0, 1, 0, 0), 0);
        assert_eq!(
            floor_sum(1_000_000_000_i64, 1, 1, 1),
            500_000_000_500_000_000
        );
        assert_eq!(
            floor_sum(1_000_000_000_i64, 1_000_000_000, 999_999_999, 999_999_999),
            499_999_999_500_000_000
        );
        assert_eq!(floor_sum(332955, 5590132, 2231, 999423), 22014575);
    }
}
