#![warn(missing_docs)]
#![doc(html_root_url="https://docs.rs/maplit/")]

//! Macros for container literals with specific type.
//!
//! ```
//! #[macro_use] extern crate maplit;
//!
//! # fn main() {
//! # use std::collections::HashMap;
//! let map: HashMap<String, i32> = hashmap!{
//!     "a" => 1,
//!     "b" => 2,
//! };
//! # }
//! ```
//!
//! The **maplit** crate uses `=>` syntax for the mapping macros. It is
//! not possible to use `:` as separator due to syntactic restrictions in
//! regular `macro_rules!` macros.
//!
//! Note that rust macros are flexible in which brackets you use for the invocation.
//! You can use them as `hashmap!{}` or `hashmap![]` or `hashmap!()`.
//! This crate suggests `{}` as the convention for the map & set macros,
//! it matches their `Debug` output.
//!
//! Generic container macros already exist elsewhere, so those are not provided
//! here at the moment.

/// Create a **HashMap** from a list of key-value pairs
///
/// ## Example
///
/// ```
/// #[macro_use] extern crate maplit;
/// # fn main() {
/// # use std::collections::HashMap;
///
/// let map: HashMap<&str, i64> = hashmap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
#[macro_export]
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                _map.insert($key.into(), $value.into());
            )*
            _map
        }
    };
}

/// Create a **HashSet** from a list of elements.
///
/// ## Example
///
/// ```
/// #[macro_use] extern crate maplit;
/// # fn main() {
/// # use std::collections::HashSet;
///
/// let set: HashSet<&str> = hashset!{"a", "b"};
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
#[macro_export]
macro_rules! hashset {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashset!(@single $rest)),*]));

    ($($key:expr,)+) => { hashset!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = hashset!(@count $($key),*);
            let mut _set = ::std::collections::HashSet::with_capacity(_cap);
            $(
                _set.insert($key.into());
            )*
            _set
        }
    };
}

/// Create a **BTreeMap** from a list of key-value pairs
///
/// ## Example
///
/// ```
/// #[macro_use] extern crate maplit;
/// # fn main() {
/// # use std::collections::BTreeMap;
///
/// let map: BTreeMap<&str, i32> = btreemap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
#[macro_export]
macro_rules! btreemap {
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (btreemap!($($key => $value),+));

    ( $($key:expr => $value:expr),* ) => {
        {
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                _map.insert($key.into(), $value.into());
            )*
            _map
        }
    };
}

/// Create a **BTreeSet** from a list of elements.
///
/// ## Example
///
/// ```
/// #[macro_use] extern crate maplit;
/// # fn main() {
/// # use std::collections::BTreeSet;
///
/// let set: BTreeSet<String> = btreeset!{"a", "b"};
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
#[macro_export]
macro_rules! btreeset {
    ($($key:expr,)+) => (btreeset!($($key),+));

    ( $($key:expr),* ) => {
        {
            let mut _set = ::std::collections::BTreeSet::new();
            $(
                _set.insert($key.into());
            )*
            _set
        }
    };
}

#[test]
fn test_hashmap() {
    use std::collections::HashMap;

    let names: HashMap<i32, &str> = hashmap!{
        1 => "one",
        2 => "two",
    };
    assert_eq!(names.len(), 2);
    assert_eq!(names[&1], "one");
    assert_eq!(names[&2], "two");
    assert_eq!(names.get(&3), None);

    let empty: HashMap<i32, i32> = hashmap!{};
    assert_eq!(empty.len(), 0);

    let into: HashMap<String, String> = hashmap!{
        "one"   => "two",
        "three" => "four",
    };
    assert_eq!(into.len(), 2);
    assert_eq!(into["one"], "two");
    assert_eq!(into["three"], "four");
    assert_eq!(into.get("five"), None);

    let _nested_compiles: HashMap<i32, HashMap<i32, i32>> = hashmap!{
        1 => hashmap!{0 => 1 + 2,},
        2 => hashmap!{1 => 1,},
    };
}

#[test]
fn test_btreemap() {
    use std::collections::BTreeMap;

    let names: BTreeMap<i32, &str> = btreemap!{
        1 => "one",
        2 => "two",
    };
    assert_eq!(names.len(), 2);
    assert_eq!(names[&1], "one");
    assert_eq!(names[&2], "two");
    assert_eq!(names.get(&3), None);

    let empty: BTreeMap<i32, i32> = btreemap!{};
    assert_eq!(empty.len(), 0);

    let into: BTreeMap<String, i32> = btreemap!{
        "foo" => 42,
        "bar" => 1337,
    };
    assert_eq!(into.len(), 2);
    assert_eq!(into["foo"], 42);
    assert_eq!(into["bar"], 1337);
    assert_eq!(into.get("qux"), None);

    let _nested_compiles: BTreeMap<i32, BTreeMap<i32, i32>> = btreemap!{
        1 => btreemap!{0 => 1 + 2,},
        2 => btreemap!{1 => 1,},
    };
}

#[test]
fn test_hashset() {
    use std::collections::HashSet;

    let settie: HashSet<i32> = hashset![256, 2, -7, 0];
    assert_eq!(settie.len(), 4);

    assert!(settie.contains(&-7));
    assert!(settie.contains(&2));
    assert!(settie.contains(&0));
    assert!(settie.contains(&256));

    assert!(!settie.contains(&7));
    assert!(!settie.contains(&-2));
    assert!(!settie.contains(&999));
    assert!(!settie.contains(&-1));

    let empty: HashSet<()> = hashset!{};
    assert_eq!(empty.len(), 0);

    let into: HashSet<char> = hashset!{
        97u8, // 'a'
        32u8, // ' '
        90u8, // 'Z'
    };
    assert_eq!(into.len(), 3);

    assert!(into.contains(&'a'));
    assert!(into.contains(&'Z'));
    assert!(into.contains(&' '));

    assert!(!into.contains(&'b'));
    assert!(!into.contains(&'#'));
    assert!(!into.contains(&'\n'));
}

#[test]
fn test_btreeset() {
    use std::collections::BTreeSet;

    let fruits: BTreeSet<&str> = btreeset!["apple", "banana", "orange"];
    assert_eq!(fruits.len(), 3);

    assert!(fruits.contains("apple"));
    assert!(fruits.contains("orange"));
    assert!(fruits.contains("banana"));

    assert!(!fruits.contains("potato"));
    assert!(!fruits.contains("carrot"));
    assert!(!fruits.contains("cucumber"));

    let empty: BTreeSet<usize> = btreeset!{};
    assert_eq!(empty.len(), 0);

    let into: BTreeSet<Option<i32>> = btreeset!{
        9, -17, 0
    };
    assert_eq!(into.len(), 3);

    assert!(into.contains(&Some(9)));
    assert!(into.contains(&Some(0)));
    assert!(into.contains(&Some(-17)));

    assert!(!into.contains(&Some(17)));
    assert!(!into.contains(&Some(65536)));
    assert!(!into.contains(&None));

    let _nested_compiles: BTreeSet<BTreeSet<BTreeSet<i32>>> = btreeset![
        btreeset!{
            btreeset!(1, 2, 3)
        },
        btreeset!{},
    ];
}

#[test]
fn test_complex() {
    use std::collections::{ HashMap, HashSet, BTreeMap, BTreeSet };

    let bits: BTreeMap<i8, HashSet<&str>> = btreemap!{
        1 => hashset!["one",  "true",  "yes"],
        0 => hashset!["zero", "false", "no" ],
    };
    assert_eq!(bits.len(), 2);
    assert_eq!(bits.keys().collect::<Vec<_>>(), vec![&0, &1]); // order matters
    assert_eq!(bits[&0], hashset!("no", "false", "zero"));

    assert!(bits.contains_key(&1));
    assert!(!bits.contains_key(&2));

    let octal: HashMap<i8, BTreeSet<String>> = hashmap!{
        0 => btreeset!["zero",  "nul"   ],
        1 => btreeset!["one",   "un"    ],
        2 => btreeset!["two",   "deux"  ],
        3 => btreeset!["three", "trois" ],
        4 => btreeset!["four",  "quatre"],
        5 => btreeset!["five",  "cinq"  ],
        6 => btreeset!["six",   "six"   ],
        7 => btreeset!["seven", "sept"  ],
    };
    assert_eq!(octal.len(), 8);
    assert_eq!(octal[&3], btreeset!("trois", "three"));

    assert!(octal.contains_key(&0));
    assert!(octal.contains_key(&7));

    assert!(!octal.contains_key(&-1));
    assert!(!octal.contains_key(&8));
}
