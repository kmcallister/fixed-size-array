/// Trait for working with fixed size arrays.
///
/// Currently this provides implementations from size 0 to 32
/// inclusive, same as some standard library traits. Once variadic
/// generics are available, this trait will probably not be needed.
pub trait FixedSizeArray {
    type Elem;
    fn as_slice<'a>(&'a self) -> &'a [Self::Elem];
    fn as_mut_slice<'a>(&'a mut self) -> &'a mut [Self::Elem];
}

macro_rules! impl_fixed_size_array {
    ( $( $n:expr )* ) => {
        $(
            impl<T> FixedSizeArray for [T; $n] {
                type Elem = T;
                
                fn as_slice<'a>(&'a self) -> &'a [T] {
                    let x: &[_] = self;
                    x
                }
                
                fn as_mut_slice<'a>(&'a mut self) -> &'a mut [T] {
                    let x: & mut [_] = self;
                    x
                }
            }
        )*
    }
}

impl_fixed_size_array! {
    0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17
    18 19 20 21 22 23 24 25 26 27 28 29 30 31 32
}

#[cfg(test)]
mod test {
    use super::FixedSizeArray;

    #[test]
    fn smoke_test() {
        assert_eq!([1, 2, 3], FixedSizeArray::as_slice(&[1, 2, 3]));
    }
}
