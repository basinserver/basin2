#[macro_export]
macro_rules! hierarchy {
    { child<$super:ident> enum $name:ident { $($item:ident, )* } } => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $name {
            $($item($item), )*
        }
        $(
            impl From<$item> for $name {
                fn from(item: $item) -> $name {
                    $name::$item(item)
                }
            }

            impl From<$item> for $super {
                fn from(item: $item) -> $super {
                    $name::from(item).into()
                }
            }
        )*
    };
    { enum $name:ident { $($item:ident, )* } } => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $name {
            $($item($item), )*
        }
        $(
            impl From<$item> for $name {
                fn from(item: $item) -> $name {
                    $name::$item(item)
                }
            }
        )*
    }
}
