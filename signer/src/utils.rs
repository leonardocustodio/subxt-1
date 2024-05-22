// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#![allow(unused_macros)]

/// Use like:
///
/// ```rust,ignore
/// once_static_cloned!{
///     /// Some documentation.
///     fn foo() -> Vec<u8> {
///         vec![1,2,3,4]
///     }
/// }
/// ```
///
/// Clones the item out of static storage. Useful if it
/// takes a while to create the item but cloning it is fairly cheap.
macro_rules! once_static_cloned {
    ($($(#[$attr:meta])* $vis:vis fn $name:ident() -> $ty:ty { $expr:expr } )+) => {
        $(
            $(#[$attr])*
            $vis fn $name() -> $ty {
                cfg_if::cfg_if! {
                    if #[cfg(feature = "std")] {
                        static VAR: std::sync::OnceLock<$ty> = std::sync::OnceLock::new();
                        VAR.get_or_init(|| { $expr }).clone()
                    } else {
                        { $expr }
                    }
                }
            }
        )+
    };
}

use core::fmt::{self, Debug, Display};

#[derive(PartialEq)]
pub struct DisplayError<T>(pub T);

impl<T> Debug for DisplayError<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Display for DisplayError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> snafu::Error for DisplayError<T> where T: Display + Debug {}
