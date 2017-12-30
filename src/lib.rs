extern crate typenum;
extern crate num_traits;

use typenum::Unsigned as CompileTimeUnsigned;
use num_traits::ops::wrapping::{
    WrappingAdd,
    WrappingMul
};
use num_traits::{
    Num, 
    Zero, 
    One, 
    Unsigned as UnsignedNum
};

use std::marker::PhantomData;
use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Rem,
};

pub trait FromCompileTimeUnsinged {
    fn from<T: CompileTimeUnsigned>() -> Self;
}

impl FromCompileTimeUnsinged for u8 {
    fn from<T: CompileTimeUnsigned>() -> Self {
        T::to_u8()
    }
}

impl FromCompileTimeUnsinged for u16 {
    fn from<T: CompileTimeUnsigned>() -> Self {
        T::to_u16()
    }
}

impl FromCompileTimeUnsinged for u32 {
    fn from<T: CompileTimeUnsigned>() -> Self {
        T::to_u32()
    }
}

impl FromCompileTimeUnsinged for u64 {
    fn from<T: CompileTimeUnsigned>() -> Self {
        T::to_u64()
    }
}

impl FromCompileTimeUnsinged for usize {
    fn from<T: CompileTimeUnsigned>() -> Self {
        T::to_usize()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Unsigned<V, M> {
    value: V,
    phantom: PhantomData<M>
}

impl<V, M> Unsigned<V, M> {
    pub fn value(self) -> V {
        self.value
    }
}

impl<V, M> From<V> for Unsigned<V, M>
    where V: Rem<Output=V> + FromCompileTimeUnsinged,
          M: CompileTimeUnsigned
{
    fn from(value: V) -> Self {
        Unsigned {
            value: value % V::from::<M>(),
            phantom: Default::default()
        }

    }
}

impl<V, M> Add for Unsigned<V, M>
    where V: WrappingAdd + Rem<Output=V> + FromCompileTimeUnsinged,
          M: CompileTimeUnsigned
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Unsigned {
            value: self.value.wrapping_add(&rhs.value) % V::from::<M>(),
            phantom: Default::default()
        }
    }
}

impl<V, M> Sub for Unsigned<V, M>
    where V: Sub<Output=V> + Ord + Rem<Output=V> + FromCompileTimeUnsinged,
          M: CompileTimeUnsigned
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let value = if self.value >= rhs.value {
            self.value - rhs.value
        } else {
            V::from::<M>() - (rhs.value - self.value)
        };

        Unsigned {
            value,
            phantom: Default::default()
        }
    }
}

impl<V, M> Mul for Unsigned<V, M>
    where V: WrappingMul + Rem<Output=V> + FromCompileTimeUnsinged,
          M: CompileTimeUnsigned
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Unsigned {
            value: self.value.wrapping_mul(&rhs.value) % V::from::<M>(),
            phantom: Default::default()
        }
    }
}

impl<V, M> Div for Unsigned<V, M>
    where V: Div<Output=V> + FromCompileTimeUnsinged,
          M: CompileTimeUnsigned
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Unsigned {
            value: self.value / rhs.value,
            phantom: Default::default()
        }
    }
}

impl<V, M> Rem for Unsigned<V, M>
    where V: Rem<Output=V> + FromCompileTimeUnsinged,
          M: CompileTimeUnsigned
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Unsigned {
            value: self.value % rhs.value,
            phantom: Default::default()
        }
    }
}

impl<V, M> Zero for Unsigned<V, M>
    where V: Zero + WrappingAdd + Rem<Output=V> + FromCompileTimeUnsinged,
          M: CompileTimeUnsigned
{
    fn zero() -> Self {
        Unsigned {
            value: V::zero(),
            phantom: Default::default()
        }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl<V, M> One for Unsigned<V, M>
    where V: One + WrappingMul + Rem<Output=V> + FromCompileTimeUnsinged,
          M: CompileTimeUnsigned
{
    fn one() -> Self {
        V::one().into()
    }
}

impl<V, M> Num for Unsigned<V, M>
    where V: Num + FromCompileTimeUnsinged + WrappingAdd + WrappingMul + PartialEq + Ord,
          M: CompileTimeUnsigned + PartialEq
{
    type FromStrRadixErr = <V as Num>::FromStrRadixErr;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(V::from_str_radix(s, radix)?.into())
    }
}

impl<V, M> UnsignedNum for Unsigned<V, M>
    where V: Num + FromCompileTimeUnsinged + WrappingAdd + WrappingMul + PartialEq + Ord,
          M: CompileTimeUnsigned + PartialEq
{}

#[cfg(test)]
mod tests {
    use typenum::U17;
    use super::Unsigned;

    type M17 = Unsigned<u8, U17>;

    #[test]
    fn construct() {
        assert_eq!(M17::from(1 ).value(), 1);
        assert_eq!(M17::from(4 ).value(), 4);
        assert_eq!(M17::from(17).value(), 0);
        assert_eq!(M17::from(18).value(), 1);
    }

    #[test]
    fn add() {
        assert_eq!(M17::from(1)  + 1 .into(), 2.into());
        assert_eq!(M17::from(16) + 1 .into(), 0.into());
        assert_eq!(M17::from(1)  + 17.into(), 1.into());
        assert_eq!(M17::from(1)  + 35.into(), 2.into());
    }

    #[test]
    fn sub() {
        assert_eq!(M17::from(2)  - 1 .into(), 1 .into());
        assert_eq!(M17::from(1)  - 3 .into(), 15.into());
        assert_eq!(M17::from(15) - 16.into(), 16.into());
        assert_eq!(M17::from(11) - 17.into(), 11.into());
    }

    #[test]
    fn mul() {
        assert_eq!(M17::from(2) * 2.into(), 4.into());
        assert_eq!(M17::from(5) * 4.into(), 3.into());
    }
}