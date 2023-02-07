use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Neg, Sub};

pub mod port;
pub mod txn;

macro_rules! discrete_newtype {
    ($id:ident, $underlying:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        pub struct $id(pub $underlying);

        impl From<$underlying> for $id {
            fn from(value: $underlying) -> Self {
                Self(value)
            }
        }

        impl From<&$id> for $id {
            fn from(value: &$id) -> Self {
                value.clone()
            }
        }
    };
}

macro_rules! scalar_newtype {
    ($id:ident, $underlying:ty) => {
        #[derive(
            Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        pub struct $id(pub $underlying);

        impl From<&$id> for $id {
            fn from(value: &$id) -> Self {
                value.clone()
            }
        }

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

        impl Neg for $id {
            type Output = $id;
            fn neg(self) -> Self::Output {
                $id(self.0.neg())
            }
        }

        impl Neg for &$id {
            type Output = $id;
            fn neg(self) -> Self::Output {
                $id(self.0.neg())
            }
        }
    };
}

discrete_newtype!(Account, String);
discrete_newtype!(Commodity, String);
discrete_newtype!(Date, String);
discrete_newtype!(PortId, String);
scalar_newtype!(CommodityPrice, Decimal);
scalar_newtype!(CommodityAmount, Decimal);
