//! Graph macros.

#[macro_export]
macro_rules! copyclone {
    ($name:ident) => {
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }
    };
}

#[macro_export]
macro_rules! clone_fields {
    ($name:ident, $($field:ident),+ $(,)*) => (
        fn clone(&self) -> Self {
            $name {
                $(
                    $field : self . $field .clone()
                ),*
            }
        }
    );
}

#[macro_export]
macro_rules! iterator_wrap {
    ($name: ident <$($typarm:tt),*> where { $($bounds: tt)* }
     item: $item: ty,
     iter: $iter: ty,
     ) => (
        pub struct $name <$($typarm),*> where $($bounds)* {
            iter: $iter,
        }
        impl<$($typarm),*> Iterator for $name <$($typarm),*>
            where $($bounds)*
        {
            type Item = $item;
            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.iter.size_hint()
            }
        }
    );
}
