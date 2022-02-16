#[macro_export]

/// Create a **HashMap** from a list of key-value pairs
///
/// uses `=>` syntax to separate the key and value for the mapping.
///
/// For example:
///     Usage:
///     hashmap!('a' => 3, 'b' => 11, 'z' => 32).
///
///     Translation:
///     {
//          let mut hm = HashMap::new();
//          hm.insert('a', 3);
//          hm.insert('b', 11);
//          hm.insert('z', 32);
//          hm
//      }
macro_rules! hashmap {
    (@single $($x:tt)*) => (());

    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    // trailing commas
    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };

    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}
