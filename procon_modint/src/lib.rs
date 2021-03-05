extern crate __procon_math_traits as math_traits;

use std::cell::RefCell;
use std::marker::PhantomData;
use std::mem::swap;
use std::ops::*;

use math_traits::{One, Zero};

pub type ModInt = DynamicModInt;
pub type ModInt167772161 = StaticModInt<Mod167772161>;
pub type ModInt469762049 = StaticModInt<Mod469762049>;
pub type ModInt924844033 = StaticModInt<Mod924844033>;
pub type ModInt998244353 = StaticModInt<Mod998244353>;
pub type ModInt1012924417 = StaticModInt<Mod1012924417>;
pub type ModInt1224736769 = StaticModInt<Mod1224736769>;

type Num = i64;

pub trait ModuloInteger:
    Copy
    + Clone
    + Ord
    + Eq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + Zero
    + One
    + From<Num>
{
    type Int: Sized;
    fn modulo(&self) -> Self::Int;
}

impl Zero for DynamicModInt {
    fn zero() -> Self {
        DynamicModInt::new(0)
    }
}

impl One for DynamicModInt {
    fn one() -> Self {
        DynamicModInt::new(1)
    }
}

impl From<Num> for DynamicModInt {
    fn from(num: Num) -> Self {
        Self::new(num)
    }
}

impl ModuloInteger for DynamicModInt {
    type Int = Num;
    fn modulo(&self) -> Num {
        modulo()
    }
}

impl<M: ModuloPrimitive> Zero for StaticModInt<M> {
    fn zero() -> Self {
        Self::new(0)
    }
}

impl<M: ModuloPrimitive> One for StaticModInt<M> {
    fn one() -> Self {
        Self::new(1)
    }
}

impl<M: ModuloPrimitive> From<Num> for StaticModInt<M> {
    fn from(num: Num) -> Self {
        Self::new(num)
    }
}

impl<M: ModuloPrimitive + Ord> ModuloInteger for StaticModInt<M> {
    type Int = Num;

    fn modulo(&self) -> Self::Int {
        M::modulo()
    }
}

thread_local! {
    static MOD: RefCell<Num> = RefCell::new(0);
}

pub fn set_modint<T>(v: T)
where
    Num: From<T>,
{
    MOD.with(|x| x.replace(Num::from(v)));
}

pub fn modulo() -> Num {
    MOD.with(|x| *x.borrow())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DynamicModInt(Num);

impl DynamicModInt {
    pub fn new<T>(v: T) -> DynamicModInt
    where
        Num: From<T>,
    {
        let mut v = Num::from(v);
        let m = modulo();
        if v >= m {
            v %= m;
        }
        if v < 0 {
            v = (v % m + m) % m;
        }
        DynamicModInt(v)
    }

    fn internal_pow(&self, mut e: Num) -> DynamicModInt {
        let mut result = 1;
        let mut cur = self.0;
        let m = modulo();
        while e > 0 {
            if e & 1 == 1 {
                result *= cur;
                result %= m;
            }
            e >>= 1;
            cur = (cur * cur) % m;
        }
        DynamicModInt(result)
    }

    pub fn pow<T>(&self, e: T) -> DynamicModInt
    where
        Num: From<T>,
    {
        self.internal_pow(Num::from(e))
    }

    pub fn value(&self) -> Num {
        self.0
    }

    pub fn inv(&self) -> Self {
        let (mut a, mut b, mut u, mut v) = (self.0, modulo(), 1, 0);
        while b > 0 {
            let tmp = a / b;
            a -= tmp * b;
            swap(&mut a, &mut b);
            u -= tmp * v;
            swap(&mut u, &mut v);
        }
        DynamicModInt::new::<i64>(u)
    }
}

impl From<DynamicModInt> for Num {
    fn from(m: DynamicModInt) -> Num {
        m.value()
    }
}

impl<T> AddAssign<T> for DynamicModInt
where
    Num: From<T>,
{
    fn add_assign(&mut self, rhs: T) {
        let mut rhs = Num::from(rhs);
        let m = modulo();
        if rhs >= m {
            rhs %= m;
        }
        self.0 += rhs;
        if self.0 >= m {
            self.0 -= m;
        }
    }
}

impl<T> Add<T> for DynamicModInt
where
    Num: From<T>,
{
    type Output = DynamicModInt;
    fn add(self, rhs: T) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl<T> SubAssign<T> for DynamicModInt
where
    Num: From<T>,
{
    fn sub_assign(&mut self, rhs: T) {
        let mut rhs = Num::from(rhs);
        let m = modulo();
        if rhs >= m {
            rhs %= m;
        }
        if rhs > 0 {
            self.0 += m - rhs;
        }
        if self.0 >= m {
            self.0 -= m;
        }
    }
}

impl<T> Sub<T> for DynamicModInt
where
    Num: From<T>,
{
    type Output = DynamicModInt;
    fn sub(self, rhs: T) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

impl<T> MulAssign<T> for DynamicModInt
where
    Num: From<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        let mut rhs = Num::from(rhs);
        let m = modulo();
        if rhs >= m {
            rhs %= m;
        }
        self.0 *= rhs;
        self.0 %= m;
    }
}

impl<T> Mul<T> for DynamicModInt
where
    Num: From<T>,
{
    type Output = DynamicModInt;
    fn mul(self, rhs: T) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl<T> DivAssign<T> for DynamicModInt
where
    Num: From<T>,
{
    fn div_assign(&mut self, rhs: T) {
        let mut rhs = Num::from(rhs);
        let m = modulo();
        if rhs >= m {
            rhs %= m;
        }
        let inv = DynamicModInt(rhs).internal_pow(m - 2);
        self.0 *= inv.value();
        self.0 %= m;
    }
}

impl<T> Div<T> for DynamicModInt
where
    Num: From<T>,
{
    type Output = DynamicModInt;
    fn div(self, rhs: T) -> Self::Output {
        let mut res = self;
        res /= rhs;
        res
    }
}

pub trait ModuloPrimitive: Clone + Copy {
    fn modulo() -> Num;
    fn primitive_root() -> Num;
}

macro_rules! define_modulo_primitive {
    ($name:ident, $mod:expr, $proot:expr) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;
        impl ModuloPrimitive for $name {
            fn modulo() -> i64 {
                $mod
            }
            fn primitive_root() -> i64 {
                $proot
            }
        }
    };
}

define_modulo_primitive!(Mod924844033, 924844033, 5);
define_modulo_primitive!(Mod998244353, 998244353, 3);
define_modulo_primitive!(Mod1012924417, 1012924417, 5);
define_modulo_primitive!(Mod167772161, 167772161, 3);
define_modulo_primitive!(Mod469762049, 469762049, 3);
define_modulo_primitive!(Mod1224736769, 1224736769, 3);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StaticModInt<M>(Num, PhantomData<M>);

impl<M: ModuloPrimitive> StaticModInt<M> {
    pub fn new<T>(v: T) -> StaticModInt<M>
    where
        Num: From<T>,
    {
        let mut v = Num::from(v);
        let m = M::modulo();
        if v >= m {
            v %= m;
        }
        if v < 0 {
            v = (v % m + m) % m;
        }
        StaticModInt(v, PhantomData)
    }

    fn internal_pow(&self, mut e: Num) -> StaticModInt<M> {
        let mut result = 1;
        let mut cur = self.0;
        let m = M::modulo();
        while e > 0 {
            if e & 1 == 1 {
                result *= cur;
                result %= m;
            }
            e >>= 1;
            cur = (cur * cur) % m;
        }
        StaticModInt(result, PhantomData)
    }

    pub fn pow<T>(&self, e: T) -> StaticModInt<M>
    where
        Num: From<T>,
    {
        self.internal_pow(Num::from(e))
    }

    pub fn value(&self) -> Num {
        self.0
    }

    pub fn inv(&self) -> Self {
        let (mut a, mut b, mut u, mut v) = (self.0, M::modulo(), 1, 0);
        while b > 0 {
            let tmp = a / b;
            a -= tmp * b;
            std::mem::swap(&mut a, &mut b);
            u -= tmp * v;
            std::mem::swap(&mut u, &mut v);
        }
        StaticModInt::new::<Num>(u)
    }
}

impl<M> Neg for StaticModInt<M>
where
    M: ModuloPrimitive,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(M::modulo() - self.0)
    }
}

impl<T, M> AddAssign<T> for StaticModInt<M>
where
    Num: From<T>,
    M: ModuloPrimitive,
{
    fn add_assign(&mut self, rhs: T) {
        let mut rhs = Num::from(rhs);
        let m = M::modulo();
        if rhs >= m {
            rhs %= m;
        }
        self.0 += rhs;
        if self.0 >= m {
            self.0 -= m;
        }
    }
}

impl<M> AddAssign for StaticModInt<M>
where
    M: ModuloPrimitive,
{
    fn add_assign(&mut self, rhs: StaticModInt<M>) {
        *self += rhs.value();
    }
}

impl<T, M> Add<T> for StaticModInt<M>
where
    Num: From<T>,
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn add(self, rhs: T) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl<M> Add for StaticModInt<M>
where
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn add(self, rhs: StaticModInt<M>) -> Self::Output {
        let mut res = self;
        res += rhs.value();
        res
    }
}

impl<M> Add<StaticModInt<M>> for Num
where
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn add(self, rhs: StaticModInt<M>) -> Self::Output {
        let mut res = StaticModInt::<M>::new(self);
        res += rhs.value();
        res
    }
}

impl<T, M> SubAssign<T> for StaticModInt<M>
where
    Num: From<T>,
    M: ModuloPrimitive,
{
    fn sub_assign(&mut self, rhs: T) {
        let mut rhs = Num::from(rhs);
        let m = M::modulo();
        if rhs >= m {
            rhs %= m;
        }
        if rhs > 0 {
            self.0 += m - rhs;
        }
        if self.0 >= m {
            self.0 -= m;
        }
    }
}

impl<M> SubAssign for StaticModInt<M>
where
    M: ModuloPrimitive,
{
    fn sub_assign(&mut self, rhs: StaticModInt<M>) {
        *self -= rhs.value();
    }
}

impl<T, M> Sub<T> for StaticModInt<M>
where
    Num: From<T>,
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn sub(self, rhs: T) -> Self::Output {
        let mut res = self;
        res -= rhs;
        res
    }
}

impl<M> Sub for StaticModInt<M>
where
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn sub(self, rhs: StaticModInt<M>) -> Self::Output {
        let mut res = self;
        res -= rhs.value();
        res
    }
}

impl<M> Sub<StaticModInt<M>> for Num
where
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn sub(self, rhs: StaticModInt<M>) -> Self::Output {
        let mut res = StaticModInt::<M>::new(self);
        res -= rhs.value();
        res
    }
}

impl<T, M> MulAssign<T> for StaticModInt<M>
where
    Num: From<T>,
    M: ModuloPrimitive,
{
    fn mul_assign(&mut self, rhs: T) {
        let mut rhs = Num::from(rhs);
        let m = M::modulo();
        if rhs >= m {
            rhs %= m;
        }
        self.0 *= rhs;
        self.0 %= m;
    }
}

impl<M> MulAssign for StaticModInt<M>
where
    M: ModuloPrimitive,
{
    fn mul_assign(&mut self, rhs: StaticModInt<M>) {
        *self *= rhs.value();
    }
}

impl<T, M> Mul<T> for StaticModInt<M>
where
    Num: From<T>,
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn mul(self, rhs: T) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

impl<M> Mul for StaticModInt<M>
where
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn mul(self, rhs: StaticModInt<M>) -> Self::Output {
        let mut res = self;
        res *= rhs.value();
        res
    }
}

impl<M> Mul<StaticModInt<M>> for Num
where
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn mul(self, rhs: StaticModInt<M>) -> Self::Output {
        let mut res = StaticModInt::<M>::new(self);
        res *= rhs.value();
        res
    }
}

impl<T, M> DivAssign<T> for StaticModInt<M>
where
    Num: From<T>,
    M: ModuloPrimitive,
{
    fn div_assign(&mut self, rhs: T) {
        let mut rhs = Num::from(rhs);
        let m = M::modulo();
        if rhs >= m {
            rhs %= m;
        }
        let inv = StaticModInt::<M>(rhs, PhantomData).internal_pow(m - 2);
        self.0 *= inv.value();
        self.0 %= m;
    }
}

impl<M> DivAssign for StaticModInt<M>
where
    M: ModuloPrimitive,
{
    fn div_assign(&mut self, rhs: StaticModInt<M>) {
        *self /= rhs.value();
    }
}

impl<T, M> Div<T> for StaticModInt<M>
where
    Num: From<T>,
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn div(self, rhs: T) -> Self::Output {
        let mut res = self;
        res /= rhs;
        res
    }
}

impl<M> Div for StaticModInt<M>
where
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn div(self, rhs: StaticModInt<M>) -> Self::Output {
        let mut res = self;
        res /= rhs.value();
        res
    }
}

impl<M> Div<StaticModInt<M>> for Num
where
    M: ModuloPrimitive,
{
    type Output = StaticModInt<M>;
    fn div(self, rhs: StaticModInt<M>) -> Self::Output {
        let mut res = StaticModInt::<M>::new(self);
        res /= rhs.value();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::Uniform;
    use rand::Rng;

    const PRIMES: [i64; 3] = [1_000_000_007, 1_000_000_009, 998_244_353];

    #[test]
    fn test_add_sub() {
        let mut rng = rand::thread_rng();
        for m in &PRIMES {
            set_modint(*m);
            for _ in 0..10000 {
                let x: i64 = rng.sample(Uniform::from(0..*m));
                let y: i64 = rng.sample(Uniform::from(0..*m));
                let mx = ModInt::new(x);
                let my = ModInt::new(y);
                assert_eq!((mx + my).value(), (x + y) % *m);
                assert_eq!((mx + y).value(), (x + y) % *m);
                assert_eq!((mx - my).value(), (x + *m - y) % *m);
                assert_eq!((mx - y).value(), (x + *m - y) % *m);
                let mut x = x;
                let mut mx = mx;
                x += y;
                mx += my;
                assert_eq!(mx.value(), x % *m);
                x += y;
                mx += y;
                assert_eq!(mx.value(), x % *m);
                x = (x + *m - y % *m) % *m;
                mx -= my;
                assert_eq!(mx.value(), x);
                x = (x + *m - y % *m) % *m;
                mx -= y;
                assert_eq!(mx.value(), x);
            }
        }
    }

    #[test]
    fn test_mul() {
        let mut rng = rand::thread_rng();
        for m in &PRIMES {
            set_modint(*m);
            for _ in 0..10000 {
                let x: i64 = rng.sample(Uniform::from(0..*m));
                let y: i64 = rng.sample(Uniform::from(0..*m));
                let mx = ModInt::new(x);
                let my = ModInt::new(y);
                assert_eq!((mx * my).value(), (x * y) % *m);
                assert_eq!((mx * y).value(), (x * y) % *m);
            }
        }
    }

    #[test]
    fn test_zero() {
        set_modint(1_000_000_007i64);
        let a = ModInt::new(1_000_000_000i64);
        let b = ModInt::new(7i64);
        let c = a + b;
        assert_eq!(c.value(), 0);
    }

    #[test]
    fn test_pow() {
        set_modint(1_000_000_007i64);
        let a = ModInt::new(1_000_000i64);
        let a = a.pow(2i64);
        assert_eq!(a.value(), 999993007);
    }

    #[test]
    fn test_div() {
        set_modint(1_000_000_007i64);
        for i in 1..=100_000i64 {
            let mut a = ModInt::new(1i64);
            a /= i;
            a *= i;
            assert_eq!(a.value(), 1);
        }
    }

    #[test]
    fn test_invmod() {
        set_modint(7i64);
        assert_eq!(ModInt::new(3i64).inv().value(), 5);
        set_modint(429i64);
        assert_eq!(ModInt::new(2i64).inv().value(), 215);
        set_modint(1_000_000_007i64);
        assert_eq!(ModInt::new(123_456_789i64).inv().value(), 18_633_540);
    }
}
