extern crate __procon_modint as modint;
extern crate __procon_ntt as ntt;

use modint::{ModuloPrimitive, StaticModInt};
use ntt::NumberTheoreticTransform;
use std::{
    cmp::min,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Rem,
        RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
};

#[derive(Debug)]
pub struct FormalPowerSeries<M: ModuloPrimitive>(Vec<StaticModInt<M>>);

impl<M: ModuloPrimitive> Clone for FormalPowerSeries<M> {
    fn clone(&self) -> FormalPowerSeries<M> {
        Self(self.0.clone())
    }
}

impl<M: ModuloPrimitive> From<Vec<StaticModInt<M>>> for FormalPowerSeries<M> {
    fn from(v: Vec<StaticModInt<M>>) -> Self {
        Self(v)
    }
}

impl<M: ModuloPrimitive> FormalPowerSeries<M> {
    pub fn new(size: usize) -> Self {
        let v = vec![StaticModInt::<M>::new(0); size];
        v.into()
    }

    pub fn from_vec<T: Copy + Into<i64>>(v: Vec<T>) -> Self {
        let mv = v
            .iter()
            .map(|&v| StaticModInt::<M>::new(v.into()))
            .collect::<Vec<_>>();
        mv.into()
    }

    pub fn values(&self) -> Vec<i64> {
        self.iter().map(|&v| v.value()).collect()
    }

    pub fn differential(&self) -> Self {
        let n = self.len();
        let mut f = Self::new(n - 1);
        for i in 1..n {
            f[i - 1] = self[i] * i as i64;
        }
        f
    }

    pub fn integral(&self) -> Self {
        let n = self.len();
        let mut f = Self::new(n + 1);
        for i in 0..n {
            f[i + 1] = self[i] / (i + 1) as i64;
        }
        f
    }

    pub fn inverse_with(&self, degree: isize) -> Self {
        assert!(self[0].value() != 0);
        let degree = if degree < 0 {
            self.len()
        } else {
            degree as usize
        };
        let mut v: Self = vec![StaticModInt::<M>::new(1) / self[0]].into();
        let mut i = 1;
        while i < degree {
            let va = v.clone() + v.clone();
            let vm = v.clone() * v.clone();
            v = (va - vm * self.head(i << 1)).head(i << 1);
            i <<= 1;
        }
        v.resize(degree, StaticModInt::<M>::new(0));
        v
    }

    pub fn inverse(&self) -> Self {
        self.inverse_with(self.len() as isize)
    }

    pub fn log_with(&self, degree: isize) -> Self {
        assert!(self[0].value() == 1);
        let mut v = (self.differential() * self.inverse_with(degree)).integral();
        v.resize(degree as usize, StaticModInt::<M>::new(0));
        v
    }

    pub fn log(&self) -> Self {
        self.log_with(self.len() as isize)
    }

    pub fn exp_with(&self, degree: isize) -> Self {
        assert!(self[0].value() == 0);
        let one = StaticModInt::<M>::new(1);
        let mut v: Self = vec![one].into();
        let mut i = 1;
        let degree = degree as usize;
        while i < degree {
            v = v.clone() * (self.head(i << 1) - v.log_with((i << 1) as isize) + one).head(i << 1);
            i <<= 1;
        }
        v.resize(degree as usize, StaticModInt::<M>::new(0));
        v
    }

    pub fn exp(&self) -> Self {
        self.exp_with(self.len() as isize)
    }

    pub fn pow_with(&self, power: usize, degree: isize) -> Self {
        let mut i = 0;
        let degree = degree as usize;
        while i < self.len() && self[i].value() == 0 {
            i += 1;
        }
        if i == self.len() || i * power >= degree {
            return Self::new(degree);
        }
        let k = self[i];
        let n = StaticModInt::<M>::new(power as i64);
        let mut v = ((((self.clone() >> i) / k).log_with(degree as isize) * n)
            .exp_with(degree as isize)
            * k.pow(power as i64))
            << (power * i);
        v.resize(degree as usize, StaticModInt::<M>::new(0));
        v
    }

    pub fn pow(&self, n: usize) -> Self {
        self.pow_with(n, self.len() as isize)
    }
}

impl<M: ModuloPrimitive> Deref for FormalPowerSeries<M> {
    type Target = Vec<StaticModInt<M>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M: ModuloPrimitive> DerefMut for FormalPowerSeries<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<M: ModuloPrimitive> Index<usize> for FormalPowerSeries<M> {
    type Output = StaticModInt<M>;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<M: ModuloPrimitive> IndexMut<usize> for FormalPowerSeries<M> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl<M: ModuloPrimitive> Neg for FormalPowerSeries<M> {
    type Output = Self;

    fn neg(self) -> Self {
        let mut v = self.clone();
        v.iter_mut().for_each(|v| *v *= -1);
        v
    }
}

impl<M: ModuloPrimitive> AddAssign<StaticModInt<M>> for FormalPowerSeries<M> {
    fn add_assign(&mut self, v: StaticModInt<M>) {
        if self.is_empty() {
            self.resize(1, StaticModInt::<M>::new(0));
        }
        self[0] += v;
    }
}

impl<M: ModuloPrimitive> AddAssign<Self> for FormalPowerSeries<M> {
    fn add_assign(&mut self, v: Self) {
        if v.len() > self.len() {
            self.resize(v.len(), StaticModInt::<M>::new(0));
        }
        for i in 0..v.len() {
            self[i] += v[i];
        }
        self.cut();
    }
}

impl<M: ModuloPrimitive> SubAssign<StaticModInt<M>> for FormalPowerSeries<M> {
    fn sub_assign(&mut self, v: StaticModInt<M>) {
        if self.is_empty() {
            self.resize(1, StaticModInt::<M>::new(0));
        }
        self[0] -= v;
    }
}

impl<M: ModuloPrimitive> SubAssign<Self> for FormalPowerSeries<M> {
    fn sub_assign(&mut self, v: Self) {
        if v.len() > self.len() {
            self.resize(v.len(), StaticModInt::<M>::new(0));
        }
        for i in 0..v.len() {
            self[i] -= v[i];
        }
        self.cut();
    }
}

impl<M: ModuloPrimitive> MulAssign<StaticModInt<M>> for FormalPowerSeries<M> {
    fn mul_assign(&mut self, v: StaticModInt<M>) {
        for i in 0..self.len() {
            self[i] *= v;
        }
    }
}

impl<M: ModuloPrimitive> MulAssign<Self> for FormalPowerSeries<M> {
    fn mul_assign(&mut self, v: Self) {
        let v = NumberTheoreticTransform::<i64, M>::multiply(&self, &v);
        self.0 = v;
    }
}

impl<M: ModuloPrimitive> DivAssign<StaticModInt<M>> for FormalPowerSeries<M> {
    fn div_assign(&mut self, v: StaticModInt<M>) {
        let inv = 1 / v;
        for i in 0..self.len() {
            self[i] /= inv;
        }
    }
}

impl<M: ModuloPrimitive> DivAssign<Self> for FormalPowerSeries<M> {
    fn div_assign(&mut self, v: Self) {
        assert!(!v.is_empty());
        assert!(v.last().copied().unwrap().value() != 0);
        self.cut();
        if self.len() < v.len() {
            self.clear();
            return;
        }
        let need = self.len() - v.len() + 1;
        let v = (self.reversed().head(need) * v.reversed().inverse_with(need as isize))
            .head(need)
            .reversed();
        *self = v;
    }
}

impl<M: ModuloPrimitive> RemAssign for FormalPowerSeries<M> {
    fn rem_assign(&mut self, v: Self) {
        self.cut();
        let r = self.clone();
        let q = r / v.clone();
        *self -= q * v;
    }
}

impl<M: ModuloPrimitive> ShlAssign<usize> for FormalPowerSeries<M> {
    fn shl_assign(&mut self, n: usize) {
        let mut v = vec![StaticModInt::<M>::new(0); n];
        v.append(&mut self.0);
        self.0 = v;
    }
}

impl<M: ModuloPrimitive> ShrAssign<usize> for FormalPowerSeries<M> {
    fn shr_assign(&mut self, n: usize) {
        self.0 = self.drain(n..).collect();
    }
}

macro_rules! impl_op {
    (impl $trait: ident for $t: ty => $fn_name: ident use $assign_fn:ident) => {
        impl<M: ModuloPrimitive> $trait<$t> for FormalPowerSeries<M> {
            type Output = FormalPowerSeries<M>;
            fn $fn_name(self, u: $t) -> FormalPowerSeries<M> {
                let mut v = self;
                v.$assign_fn(u);
                v
            }
        }
    };
}

impl_op!(impl Add for StaticModInt<M> => add use add_assign);
impl_op!(impl Add for Self => add use add_assign);
impl_op!(impl Sub for StaticModInt<M> => sub use sub_assign);
impl_op!(impl Sub for Self => sub use sub_assign);
impl_op!(impl Mul for StaticModInt<M> => mul use mul_assign);
impl_op!(impl Mul for Self => mul use mul_assign);
impl_op!(impl Div for StaticModInt<M> => div use div_assign);
impl_op!(impl Div for Self => div use div_assign);
impl_op!(impl Rem for Self => rem use rem_assign);
impl_op!(impl Shl for usize => shl use shl_assign);
impl_op!(impl Shr for usize => shr use shr_assign);

impl<M: ModuloPrimitive> FormalPowerSeries<M> {
    fn cut(&mut self) {
        while !self.is_empty() && self.iter().next_back().unwrap().value() == 0 {
            self.pop();
        }
    }

    fn reversed(&self) -> Self {
        let mut v = self.clone();
        v.reverse();
        v
    }

    fn head(&self, n: usize) -> Self {
        self.clone()
            .drain(..min(n, self.len()))
            .collect::<Vec<_>>()
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modint::Mod998244353;

    type Fps = FormalPowerSeries<Mod998244353>;

    #[test]
    fn test_inv_fps() {
        let f = Fps::from_vec(vec![5, 4, 3, 2, 1]);
        let g = f.inverse();
        assert_eq!(
            g.values(),
            vec![598946612, 718735934, 862483121, 635682004, 163871793]
        );
    }

    #[test]
    fn test_exp_fps() {
        let f = Fps::from_vec(vec![0, 1, 2, 3, 4]);
        let g = f.exp();
        assert_eq!(g.values(), vec![1, 1, 499122179, 166374064, 291154613]);
    }

    #[test]
    fn test_log_fps() {
        let f = Fps::from_vec(vec![1, 1, 499122179, 166374064, 291154613]);
        let g = f.log();
        assert_eq!(g.values(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_pow_fps() {
        let f = Fps::from_vec(vec![0, 0, 9, 12]);
        let g = f.pow(3);
        assert_eq!(g.values(), vec![0; 4]);
    }
}
