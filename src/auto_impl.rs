#[macro_export]
macro_rules! auto_impl {
    (trait $trait:ident for $mod:ident $(($($field:ident),*))? {$($item:item)*}) => {
        trait $trait: $mod::Interface $($(+ $mod::has::$field)*)? {
            $($item)*
        }
        impl<T: $mod::Interface $($(+ $mod::has::$field)*)?> $trait for T {}
    };
}

// macro_rules! inner {
//     ($trait:ident $mod:ident $($item:item)* => $($bounds:tt)*) => {
//         trait $trait: $mod::Interface $($bounds:tt)* {
//             $($item)*
//         }
//         impl<T: $mod::Interface $($bounds:tt)*> $trait for T {}
//     };
//     ($trait:ident $mod:ident $($item:item)* $field:ident $($remaining:tt)* => $($bounds:tt)*) => {
//         inner!($trait $mod $($item)* $($remaining)* => $($bounds)* + $mod::has::$field)
//     }
// }
