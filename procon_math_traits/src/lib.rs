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
    fn minimum() -> Self;
}

pub trait BoundedAbove {
    fn maximum() -> Self;
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
        floor ceil round trunc fract abs signum sqrt
        exp exp2 ln log2 log10 cbrt sin cos tan
        asin acos atan exp_m1 ln_1p sinh cosh tanh
        asinh acosh atanh recip to_degrees to_radians
    );

    fn cast_f32(v: f32) -> Self;
    fn cast_f64(v: f64) -> Self;

    fn sin_cos(self) -> (Self, Self);
    fn atan2(self, rhs: Self) -> Self;
    fn hypot(self, rhs: Self) -> Self;

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
            fn minimum() -> Self {
                Self::min_value()
            }
        }

        impl BoundedAbove for $t {
            fn maximum() -> Self {
                Self::max_value()
            }
        }

        impl PrimitiveInteger for $t {}
    )*}
}

macro_rules! forward {
    ($( Self :: $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
    => {$(
        #[inline]
        fn $method(self $( , $arg : $ty )* ) -> $ret {
            Self::$method(self $( , $arg )* )
        }
    )*};
    ($( $base:ident :: $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ; )*)
    => {$(
        #[inline]
        fn $method(self $( , $arg : $ty )* ) -> $ret {
            <Self as $base>::$method(self $( , $arg )* )
        }
    )*};
    ($( $base:ident :: $method:ident ( $( $arg:ident : $ty:ty ),* ) -> $ret:ty ; )*)
    => {$(
        #[inline]
        fn $method( $( $arg : $ty ),* ) -> $ret {
            <Self as $base>::$method( $( $arg ),* )
        }
    )*}
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
            forward! {
                Self::floor(self) -> Self;
                Self::ceil(self) -> Self;
                Self::round(self) -> Self;
                Self::trunc(self) -> Self;
                Self::fract(self) -> Self;
                Self::abs(self) -> Self;
                Self::signum(self) -> Self;
                Self::recip(self) -> Self;
                Self::sqrt(self) -> Self;
                Self::exp(self) -> Self;
                Self::exp2(self) -> Self;
                Self::ln(self) -> Self;
                Self::log2(self) -> Self;
                Self::log10(self) -> Self;
                Self::to_degrees(self) -> Self;
                Self::to_radians(self) -> Self;
                Self::cbrt(self) -> Self;
                Self::hypot(self, other: Self) -> Self;
                Self::sin(self) -> Self;
                Self::cos(self) -> Self;
                Self::tan(self) -> Self;
                Self::asin(self) -> Self;
                Self::acos(self) -> Self;
                Self::atan(self) -> Self;
                Self::atan2(self, other: Self) -> Self;
                Self::sin_cos(self) -> (Self, Self);
                Self::exp_m1(self) -> Self;
                Self::ln_1p(self) -> Self;
                Self::sinh(self) -> Self;
                Self::cosh(self) -> Self;
                Self::tanh(self) -> Self;
                Self::asinh(self) -> Self;
                Self::acosh(self) -> Self;
                Self::atanh(self) -> Self;
            }

            fn eps() -> Self { std::$t::EPSILON }
            fn pi() -> Self { std::$t::consts::PI }
            fn pi_deg() -> Self { 180.0 }
            fn tau() -> Self { std::$t::consts::PI * 2.0 }
            fn tau_deg() -> Self { 360.0 }

            fn cast_f32(v: f32) -> $t {
                v as $t
            }

            fn cast_f64(v: f64) -> $t {
                v as $t
            }
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
