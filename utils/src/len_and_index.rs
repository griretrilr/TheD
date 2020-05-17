extern crate len_trait;

use len_trait::Len;
use std::ops::Index;

/// Trait you can implement on any type that implements the `Len` and `Index<usize>` traits to get an 'iter' function implemented against those base traits.
pub trait LenAndIndex<'a, Item>: Len + Index<usize, Output = Item> where Item: 'a { }

mod iterators {
    use std::marker::PhantomData;

    /// `Iterator` for a type that implements the `LenAndIndex` trait.
    pub struct Iter<'a, Collection, Item> 
    where 
        Collection: super::LenAndIndex<'a, Item>,
        Item: 'a {
        collection: &'a Collection,
        next: usize,
        item: PhantomData<&'a Item>,
    }

    impl<'a, Collection, Item> Iter<'a, Collection, Item>
    where Collection: super::LenAndIndex<'a, Item> {
        pub fn new(collection: &'a Collection) -> Iter<'a, Collection, Item> {
            Iter { 
                collection, 
                next: 0,
                item: PhantomData,
            }
        }
    }

    impl<'a, Collection, Item> Iterator for Iter<'a, Collection, Item> 
    where Collection: super::LenAndIndex<'a, Item> {
        type Item = &'a Item;

        fn next(&mut self) -> Option<Self::Item> {
            if self.next >= self.collection.len() {
                None
            } else {
                let result = self.collection.index(self.next);
                self.next += 1;
                Some(result)
            }
        }
    }
}

pub trait Iterable<'a, Item>: LenAndIndex<'a, Item>
where Item: 'a {
    type Iter;

    fn iter(&'a self) -> Self::Iter;
}

impl<'a, Collection, Item> Iterable<'a, Item> for Collection
where 
    Collection: LenAndIndex<'a, Item> + 'a,
    Item: 'a {
    type Iter = iterators::Iter<'a, Collection, Item>;

    fn iter(&'a self) -> Self::Iter {
        iterators::Iter::new(self)
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;

    use super::*;
    use rstest::rstest;

    mod mocks {
        use len_trait::{Empty, Len};
        use std::ops::Index;

        pub struct LenAndIndex<T>(pub Vec<T>);

        impl<T> Empty for LenAndIndex<T> {
            fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl<T> Len for LenAndIndex<T> {
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl<'a, T> Index<usize> for LenAndIndex<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                self.0.index(index)
            }
        }

        impl<'a, T> crate::len_and_index::LenAndIndex<'a, T> for LenAndIndex<T>
        where T: 'a { }
    }

    #[rstest(items,
        case(&[]),
        case(&[1]),
        case(&[1, 2, 3]),
    )]
    fn test_iter(items: &[i32]) {
        let collection = mocks::LenAndIndex(Vec::from(items));
        let mut iter = collection.iter();
        let len = items.len();
        for i in 0..len {
            assert_eq!(iter.next(), Some(&items[i]));
        }
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}