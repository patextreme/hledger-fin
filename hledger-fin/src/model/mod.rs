pub mod spot;

use rust_decimal::Decimal;
use std::ops::{Add, Sub};

macro_rules! discrete_newtype {
    ($id:ident, $underlying:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $id(pub $underlying);

        impl From<$underlying> for $id {
            fn from(value: $underlying) -> Self {
                Self(value)
            }
        }
    };
}

macro_rules! scalar_newtype {
    ($id:ident, $underlying:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $id(pub $underlying);

        impl From<$underlying> for $id {
            fn from(value: $underlying) -> Self {
                Self(value)
            }
        }

        impl Add for $id {
            type Output = $id;
            fn add(self, rhs: Self) -> Self::Output {
                $id(self.0 + rhs.0)
            }
        }

        impl Add for &$id {
            type Output = $id;
            fn add(self, rhs: Self) -> Self::Output {
                $id(self.0 + rhs.0)
            }
        }

        impl Sub for $id {
            type Output = $id;
            fn sub(self, rhs: Self) -> Self::Output {
                $id(self.0 - rhs.0)
            }
        }

        impl Sub for &$id {
            type Output = $id;
            fn sub(self, rhs: Self) -> Self::Output {
                $id(self.0 - rhs.0)
            }
        }
    };
}

discrete_newtype!(Account, String);
discrete_newtype!(Commodity, String);
discrete_newtype!(Date, String);
scalar_newtype!(Price, Decimal);
scalar_newtype!(Volume, Decimal);
