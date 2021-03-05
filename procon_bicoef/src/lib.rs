extern crate __procon_modint as modint;

use modint::{DynamicModInt, ModuloInteger};

pub type ModBicoef = Bicoef<DynamicModInt>;

#[derive(Debug)]
pub enum Bicoef<M: ModuloInteger> {
    SmallN(SmallNCombination<M>),
    LargeN(LargeNCombination<M>),
}

impl<M: ModuloInteger> Bicoef<M> {
    pub fn small_new(n: usize) -> Bicoef<M> {
        let c = SmallNCombination::<M>::new(n);
        Bicoef::SmallN(c)
    }

    pub fn large_new(n: usize, r: usize) -> Bicoef<M> {
        let c = LargeNCombination::<M>::new(n, r);
        Bicoef::LargeN(c)
    }

    pub fn comb(&self, n: usize, r: usize) -> M {
        use Bicoef::*;
        match self {
            SmallN(c) => c.comb(n, r),
            LargeN(c) => {
                assert_eq!(c.n, n);
                c.comb(r)
            }
        }
    }
}

#[derive(Debug)]
pub struct SmallNCombination<M: ModuloInteger> {
    pub fact: Vec<M>,
    pub finv: Vec<M>,
    pub inv: Vec<M>,
}

impl<M: ModuloInteger> SmallNCombination<M> {
    pub fn new(max: usize) -> SmallNCombination<M> {
        let mut fact = vec![M::one(); max + 1];
        let mut finv = vec![M::one(); max + 1];
        let mut inv = vec![M::one(); max + 1];
        for i in 1..=max {
            fact[i] = fact[i - 1] * (i as i64).into();
        }
        finv[max] /= fact[max];
        for i in (0..max).rev() {
            finv[i] = finv[i + 1] * ((i + 1) as i64).into();
        }
        for i in 1..=max {
            inv[i] = finv[i] * fact[i - 1];
        }
        Self { fact, finv, inv }
    }

    pub fn perm(&self, n: usize, r: usize) -> M {
        if n < r {
            M::zero()
        } else {
            self.fact[n] * self.finv[n - r]
        }
    }

    pub fn comb(&self, n: usize, r: usize) -> M {
        if n < r {
            M::zero()
        } else {
            self.fact[n] * self.finv[r] * self.finv[n - r]
        }
    }

    pub fn multicomb(&self, n: usize, r: usize) -> M {
        if r == 0 {
            M::one()
        } else {
            self.comb(n + r - 1, r)
        }
    }
}

#[derive(Debug)]
pub struct LargeNCombination<M: ModuloInteger> {
    pub n: usize,
    com: Vec<M>,
}

impl<M: ModuloInteger> LargeNCombination<M> {
    pub fn new(n: usize, r_max: usize) -> LargeNCombination<M> {
        let mut fact = vec![M::one(); r_max + 1];
        let mut finv = vec![M::one(); r_max + 1];
        let mut inv = vec![M::one(); r_max + 1];
        for i in 1..=r_max {
            fact[i] = fact[i - 1] * (i as i64).into();
        }
        finv[r_max] /= fact[r_max];
        for i in (0..r_max).rev() {
            finv[i] = finv[i + 1] * ((i + 1) as i64).into();
        }
        for i in 1..=r_max {
            inv[i] = finv[i] * fact[i - 1];
        }
        let mut com = vec![M::one(); r_max + 1];
        com[0] = M::one();
        for i in 1..=r_max {
            com[i] = com[i - 1] * ((n - i + 1) as i64).into() * inv[i];
        }
        Self { n, com }
    }

    pub fn comb(&self, r: usize) -> M {
        self.com[r]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use modint::set_modint;

    #[test]
    fn test_small_comb_1() {
        set_modint(1_000_000_007_i64);
        let c = ModBicoef::small_new(15);
        assert_eq!(c.comb(12, 4).value(), 495);
    }

    #[test]
    fn test_large_comb_1() {
        set_modint(1_000_000_007_i64);
        let c = ModBicoef::large_new(12, 12);
        assert_eq!(c.comb(12, 4).value(), 495);
    }
}
