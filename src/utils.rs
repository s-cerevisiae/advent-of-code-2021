//! Some code that is repeatedly used

use std::ops::Sub;

pub fn abs_diff<T: Sub + PartialOrd>(x: T, y: T) -> <T as Sub>::Output {
    if x < y {
        y - x
    } else {
        x - y
    }
}
