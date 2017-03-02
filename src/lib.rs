#[macro_use]
extern crate combine;

macro_rules! iter {
    () => { $crate::std::iter::empty() };
    ($e:expr) => { $crate::std::iter::once($e) };
    ($e:expr, $($es:expr),+) => { $crate::std::iter::Iterator::chain($crate::std::iter::once($e),
                                                                     iter![$($es),*]) };
}

pub mod basic;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_macro() {
        assert_eq!(iter![1, 2, 3].collect::<Vec<u8>>(), vec![1, 2, 3])
    }
}
