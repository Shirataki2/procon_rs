use std::{
    fmt,
    iter::{Product, Sum},
    marker::PhantomData,
    ops::*,
};

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait Signed {
    fn abs() -> Self;
}

pub trait Unsigned {}

pub trait BoundedBelow {
    fn min() -> Self;
}

pub trait BoundedAbove {
    fn max() -> Self;
}

pub trait Monoid {
    type T: Clone;
    fn id() -> Self::T;
    fn op(lhs: &Self::T, rhs: &Self::T) -> Self::T;
}

pub trait Bounded: BoundedBelow + BoundedAbove {}

impl<T: BoundedBelow + BoundedAbove + ?Sized> Bounded for T {}

pub trait PrimitiveInteger:
    'static
    + Copy
    + Clone
    + Ord
    + Eq
    + Send
    + Sync
    + Not<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Sum
    + Product
    + Zero
    + One
    + Bounded
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + Shl<Output = Self>
    + Shr<Output = Self>
    + ShlAssign
    + ShrAssign
    + fmt::Display
    + fmt::Debug
    + fmt::Binary
    + fmt::Octal
{
}

macro_rules! fn_float {
    ($($f: ident)*) => {
        $(fn $f(self) -> Self;)*
    };
}

pub trait PrimitiveFloating:
    'static
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Send
    + Sync
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Sum
    + Product
    + Zero
    + One
    + fmt::Display
    + fmt::Debug
{
    fn_float!(
        float ceil round trunc fract abs signum sqrt
        exp exp2 ln log2 log10 cbrt sin cos tan
        asin acos atan exp_m1 ln_1p sinh cosh tanh
        asinh acosh atanh recip to_degrees to_radians
    );

    fn sin_cos(&self) -> (Self, Self);
    fn atan2(&self, rhs: Self) -> Self;
    fn hypot(&self, rhs: Self) -> Self;

    fn eps() -> Self;
    fn pi() -> Self;
    fn pi_deg() -> Self;
    fn tau() -> Self;
    fn tau_deg() -> Self;
}

pub trait Field:
    'static
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Send
    + Sync
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Sum
    + Product
    + Zero
    + One
{
}

impl<T> Field for T where
    T: 'static
        + Copy
        + Clone
        + PartialEq
        + PartialOrd
        + Send
        + Sync
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + Sum
        + Product
        + Zero
        + One
{
}

macro_rules! impl_primitive_integer {
    ($($t : ty)*) => {$(
        impl Zero for $t {
            #[inline]
            fn zero() -> Self {
                0
            }
        }

        impl One for $t {
            #[inline]
            fn one() -> Self {
                1
            }
        }

        impl BoundedBelow for $t {
            #[inline]
            fn min() -> Self {
                Self::min_value()
            }
        }

        impl BoundedAbove for $t {
            fn max() -> Self {
                Self::max_value()
            }
        }

        impl PrimitiveInteger for $t {}
    )*}
}

macro_rules! impl_float {
    ($($f: ident)*) => {
        $(
            #[allow(unconditional_recursion)]
            fn $f(self) -> Self { self.$f() }
        )*
    };
}

macro_rules! impl_primitive_floating {
    ($($t : tt)*) => {$(
        impl Zero for $t {
            #[inline]
            fn zero() -> Self {
                0.0
            }
        }

        impl One for $t {
            #[inline]
            fn one() -> Self {
                1.0
            }
        }

        impl PrimitiveFloating for $t {
            impl_float!(
                float ceil round trunc fract abs signum sqrt
                exp exp2 ln log2 log10 cbrt sin cos tan
                asin acos atan exp_m1 ln_1p sinh cosh tanh
                asinh acosh atanh recip to_degrees to_radians
            );

            #[allow(unconditional_recursion)]
            fn sin_cos(&self) -> (Self, Self) {
                self.sin_cos()
            }

            #[allow(unconditional_recursion)]
            fn atan2(&self, rhs: Self) -> Self { self.atan2(rhs) }
            #[allow(unconditional_recursion)]
            fn hypot(&self, rhs: Self) -> Self { self.hypot(rhs) }
            fn eps() -> Self { std::$t::EPSILON }
            fn pi() -> Self { std::$t::consts::PI }
            fn pi_deg() -> Self { 180.0 }
            fn tau() -> Self { std::$t::consts::PI * 2.0 }
            fn tau_deg() -> Self { 360.0 }
        }
    )*}
}

impl_primitive_integer!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);
impl_primitive_floating!(f32 f64);

pub struct Additive<T>(PhantomData<fn() -> T>);
impl<T> Monoid for Additive<T>
where
    T: Copy + Add<Output = T> + Zero,
{
    type T = T;
    fn id() -> Self::T {
        T::zero()
    }
    fn op(a: &T, b: &T) -> Self::T {
        *a + *b
    }
}

pub struct Multiplicative<T>(PhantomData<fn() -> T>);
impl<T> Monoid for Multiplicative<T>
where
    T: Copy + Mul<Output = T> + One,
{
    type T = T;
    fn id() -> T {
        T::one()
    }
    fn op(a: &T, b: &T) -> T {
        *a * *b
    }
}

pub struct Gcd<T>(PhantomData<fn() -> T>);
impl<T: PrimitiveInteger> Monoid for Gcd<T> {
    type T = T;
    fn id() -> T {
        T::zero()
    }
    fn op(a: &T, b: &T) -> T {
        let r = *a % *b;
        if r == T::zero() {
            *b
        } else {
            Self::op(b, &r)
        }
    }
}

pub struct Xor<T>(PhantomData<fn() -> T>);
impl<T: PrimitiveInteger> Monoid for Xor<T> {
    type T = T;
    fn id() -> T {
        T::zero()
    }
    fn op(a: &T, b: &T) -> T {
        *a ^ *b
    }
}
