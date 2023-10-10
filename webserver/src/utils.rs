use anyhow::{anyhow, Result};
use std::io::Error;

pub trait CheckNegativeOne {
    fn check_negative_one(&self) -> bool;
}

macro_rules! impl_check_negative_one {
    ($type:ty) => {
        impl CheckNegativeOne for $type {
            fn check_negative_one(&self) -> bool {
                *self == -1
            }
        }
    };
}

impl_check_negative_one!(i32);
impl_check_negative_one!(isize);

pub fn rv_handler<T: CheckNegativeOne>(rv: T) -> Result<T> {
    if rv.check_negative_one() {
        Err(anyhow!("{:?}", Error::last_os_error()))
    } else {
        Ok(rv)
    }
}