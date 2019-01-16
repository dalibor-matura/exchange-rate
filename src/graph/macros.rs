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
