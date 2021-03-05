extern crate __procon_complex as complex;
extern crate __procon_math_traits as math_traits;

use complex::Complex;
use math_traits::{PrimitiveFloating, Zero};

use std::ops::{Index, IndexMut, Mul};

pub struct FastFourierTransform<F: PrimitiveFloating>(Vec<F>);

impl<F: PrimitiveFloating> From<Vec<F>> for FastFourierTransform<F> {
    fn from(v: Vec<F>) -> Self {
        Self(v)
    }
}

impl<F: PrimitiveFloating> FastFourierTransform<F> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn bit_reverse<T>(v: &mut [T]) {
        let mut i = 0;
        for j in 1..v.len() - 1 {
            let mut k = v.len() >> 1;
            while {
                i ^= k;
                k > i
            } {
                k >>= 1;
            }
            if i > j {
                v.swap(i, j);
            }
        }
    }

    fn dft(f: &mut [Complex<F>], inv: bool) {
        let size = f.len();
        let pi = if inv { -F::pi() } else { F::pi() };
        Self::bit_reverse(f);
        for i in (0..).map(|i| 1 << i).take_while(|&i| i < size) {
            for k in 0..i {
                let theta = F::cast_f64(k as f64) * pi / F::cast_f64(i as f64);
                let w = Complex::from_polar(F::one(), theta);
                for j in (0..).map(|j| 2 * i * j).take_while(|&j| j < size) {
                    let s = f[j + k];
                    let t = f[j + k + i] * w;
                    f[j + k] = s + t;
                    f[j + k + i] = s - t;
                }
            }
        }
    }
}

impl<F: PrimitiveFloating> Index<usize> for FastFourierTransform<F> {
    type Output = F;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<F: PrimitiveFloating> IndexMut<usize> for FastFourierTransform<F> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<F: PrimitiveFloating> Mul for FastFourierTransform<F> {
    type Output = Vec<F>;
    fn mul(self, rhs: Self) -> Self::Output {
        let m = self.len() + rhs.len() + 1;
        let n = m.next_power_of_two();
        let mut ff = vec![Complex::<F>::zero(); n];
        let mut gg = vec![Complex::<F>::zero(); n];
        for i in 0..self.len() {
            ff[i] += self[i];
        }
        for i in 0..rhs.len() {
            gg[i] += rhs[i];
        }
        Self::dft(&mut ff, false);
        Self::dft(&mut gg, false);
        for i in 0..n {
            ff[i] *= gg[i];
        }
        Self::dft(&mut ff, true);
        let mut res = vec![F::zero(); m];
        for i in 0..m {
            res[i] = ff[i].real() / F::cast_f64(n as f64);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Fft = FastFourierTransform<f64>;

    #[test]
    fn test_fft() {
        let f: Fft = vec![0.0, 1.0, 2.0, 3.0, 4.0].into();
        let g: Fft = vec![0.0, 1.0, 2.0, 4.0, 8.0].into();
        let x = f * g;
        let x = x.iter().map(|v| v.round() as i32).collect::<Vec<_>>();
        assert_eq!(x, vec![0, 0, 1, 4, 11, 26, 36, 40, 32, 0, 0])
    }
}
