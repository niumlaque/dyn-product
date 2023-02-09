//! # DYN-PRODUCT
//! A crate that creates cartesian product of size determined at runtime.
//!
//! If the size is determined at compile time, it is better to use `itertools::iproduct!`.
//!
//! # Usage
//! Add dependency to your toml.
//! ```toml
//! [dependencies]
//! dyn-product = { git = "https://github.com/niumlaque/dyn-product", branch = "main" }
//! ```
//!
//! # Examples
//! ```no_run
//! # use crate::*;
//! use dyn_product::DynProduct;
//!
//! let data = vec![
//!     vec!["GroupA-1", "GroupA-2", "GroupA-3"],
//!     vec!["GroupB-1", "GroupB-2"],
//!     vec!["GroupC-1", "GroupC-2", "GroupC-3", "GroupC-4"],
//! ];
//!
//! for item in DynProduct::from(&data) {
//!     println!("{:?}", item);
//! }
//! ```
//! output:
//! ```text
//! ["GroupA-1", "GroupB-1", "GroupC-1"]
//! ["GroupA-1", "GroupB-1", "GroupC-2"]
//! ["GroupA-1", "GroupB-1", "GroupC-3"]
//! ["GroupA-1", "GroupB-1", "GroupC-4"]
//! ["GroupA-1", "GroupB-2", "GroupC-1"]
//! ["GroupA-1", "GroupB-2", "GroupC-2"]
//! ["GroupA-1", "GroupB-2", "GroupC-3"]
//! ["GroupA-1", "GroupB-2", "GroupC-4"]
//! ["GroupA-2", "GroupB-1", "GroupC-1"]
//! ["GroupA-2", "GroupB-1", "GroupC-2"]
//! ["GroupA-2", "GroupB-1", "GroupC-3"]
//! ["GroupA-2", "GroupB-1", "GroupC-4"]
//! ["GroupA-2", "GroupB-2", "GroupC-1"]
//! ["GroupA-2", "GroupB-2", "GroupC-2"]
//! ["GroupA-2", "GroupB-2", "GroupC-3"]
//! ["GroupA-2", "GroupB-2", "GroupC-4"]
//! ["GroupA-3", "GroupB-1", "GroupC-1"]
//! ["GroupA-3", "GroupB-1", "GroupC-2"]
//! ["GroupA-3", "GroupB-1", "GroupC-3"]
//! ["GroupA-3", "GroupB-1", "GroupC-4"]
//! ["GroupA-3", "GroupB-2", "GroupC-1"]
//! ["GroupA-3", "GroupB-2", "GroupC-2"]
//! ["GroupA-3", "GroupB-2", "GroupC-3"]
//! ["GroupA-3", "GroupB-2", "GroupC-4"]
//! ```

/// Provides a function to extract a slice.
#[allow(clippy::len_without_is_empty)]
pub trait AsSlice {
    /// Element type of the slice.
    type Item;

    /// Returns the slice of `Self`
    fn as_slice(&self) -> &[Self::Item];

    /// Returns the number of elements in the `Self`.
    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl<T> AsSlice for Vec<T> {
    type Item = T;
    fn as_slice(&self) -> &[Self::Item] {
        self
    }
}

/// An iterator that iterates over combinations of specified slices.
pub struct DynProduct<'a, T, I: AsSlice<Item = T>> {
    v: &'a [I],
    i: Vec<usize>,
}

fn countup<T, I: AsSlice<Item = T>>(i: &mut [usize], v: &[I]) {
    let last = i.len() - 1;
    i[last] += 1;

    for j in (1..=last).rev() {
        if i[j] >= v[j].len() {
            i[j] = 0;
            i[j - 1] += 1;
        } else {
            break;
        }
    }
}

impl<'a, T: 'a, I: AsSlice<Item = T>> Iterator for DynProduct<'a, T, I> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i[0] >= self.v[0].len() {
            return None;
        }

        let mut ret = Vec::with_capacity(self.v.len());
        for (i, x) in self.v.iter().map(|x| x.as_slice()).enumerate() {
            ret.push(&x[self.i[i]]);
        }

        countup(&mut self.i, self.v);
        Some(ret)
    }
}

impl<'a, T, I: AsSlice<Item = T>> From<&'a [I]> for DynProduct<'a, T, I> {
    fn from(value: &'a [I]) -> Self {
        Self {
            v: value,
            i: vec![0; value.len()],
        }
    }
}

impl<'a, T, I: AsSlice<Item = T>> From<&'a Vec<I>> for DynProduct<'a, T, I> {
    fn from(value: &'a Vec<I>) -> Self {
        Self {
            v: value,
            i: vec![0; value.len()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countup() {
        let data = vec![vec![1, 1, 1], vec![1, 1], vec![1, 1, 1]];
        let mut actual = vec![0; data.len()];

        countup(&mut actual, &data);
        assert_eq!(actual, vec![0, 0, 1]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![0, 0, 2]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![0, 1, 0]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![0, 1, 1]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![0, 1, 2]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![1, 0, 0]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![1, 0, 1]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![1, 0, 2]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![1, 1, 0]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![1, 1, 1]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![1, 1, 2]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![2, 0, 0]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![2, 0, 1]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![2, 0, 2]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![2, 1, 0]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![2, 1, 1]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![2, 1, 2]);

        countup(&mut actual, &data);
        assert_eq!(actual, vec![3, 0, 0]);
    }

    #[test]
    fn test_product() {
        #[derive(Debug, PartialEq)]
        struct Info {
            category: String,
            value: i32,
        }

        impl Info {
            fn new(category: impl Into<String>, value: i32) -> Self {
                Self {
                    category: category.into(),
                    value,
                }
            }
        }

        let data = vec![
            vec![
                Info::new("GroupA", 1),
                Info::new("GroupA", 2),
                Info::new("GroupA", 3),
            ],
            vec![Info::new("GroupB", 1), Info::new("GroupB", 2)],
            vec![
                Info::new("GroupC", 1),
                Info::new("GroupC", 2),
                Info::new("GroupC", 3),
                Info::new("GroupC", 4),
            ],
        ];

        let p: Vec<_> = DynProduct::from(&data).collect();
        assert_eq!(p.len(), data[0].len() * data[1].len() * data[2].len());
        assert_eq!(p[0], vec![&data[0][0], &data[1][0], &data[2][0]]);
        assert_eq!(p[1], vec![&data[0][0], &data[1][0], &data[2][1]]);
        assert_eq!(p[2], vec![&data[0][0], &data[1][0], &data[2][2]]);
        assert_eq!(p[3], vec![&data[0][0], &data[1][0], &data[2][3]]);
        assert_eq!(p[4], vec![&data[0][0], &data[1][1], &data[2][0]]);
        assert_eq!(p[5], vec![&data[0][0], &data[1][1], &data[2][1]]);
        assert_eq!(p[6], vec![&data[0][0], &data[1][1], &data[2][2]]);
        assert_eq!(p[7], vec![&data[0][0], &data[1][1], &data[2][3]]);
        assert_eq!(p[8], vec![&data[0][1], &data[1][0], &data[2][0]]);
        assert_eq!(p[9], vec![&data[0][1], &data[1][0], &data[2][1]]);
        assert_eq!(p[10], vec![&data[0][1], &data[1][0], &data[2][2]]);
        assert_eq!(p[11], vec![&data[0][1], &data[1][0], &data[2][3]]);
        assert_eq!(p[12], vec![&data[0][1], &data[1][1], &data[2][0]]);
        assert_eq!(p[13], vec![&data[0][1], &data[1][1], &data[2][1]]);
        assert_eq!(p[14], vec![&data[0][1], &data[1][1], &data[2][2]]);
        assert_eq!(p[15], vec![&data[0][1], &data[1][1], &data[2][3]]);
        assert_eq!(p[16], vec![&data[0][2], &data[1][0], &data[2][0]]);
        assert_eq!(p[17], vec![&data[0][2], &data[1][0], &data[2][1]]);
        assert_eq!(p[18], vec![&data[0][2], &data[1][0], &data[2][2]]);
        assert_eq!(p[19], vec![&data[0][2], &data[1][0], &data[2][3]]);
        assert_eq!(p[20], vec![&data[0][2], &data[1][1], &data[2][0]]);
        assert_eq!(p[21], vec![&data[0][2], &data[1][1], &data[2][1]]);
        assert_eq!(p[22], vec![&data[0][2], &data[1][1], &data[2][2]]);
        assert_eq!(p[23], vec![&data[0][2], &data[1][1], &data[2][3]]);
    }
}
