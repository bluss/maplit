#![warn(missing_docs)]

//! Macros for container literals with specific type.
//! 
//! ```
//! #[macro_use]
//! extern crate maplit;
//!
//! # fn main() {
//! let map = hashmap!{
//!     "a" => 1,
//!     "b" => 2,
//! };
//! # }
//! ```
//!
//! The **maplit** crate uses `=>` syntax for the mapping macros. It is
//! not possible to use `:` as separator due to syntactic the restrictions in
//! regular `macro_rules!` macros.
//!
//! Note that rust macros are flexible in which brackets you use for the invocation.
//! You can use them as `hashmap!{}` or `hashmap![]` or `hashmap!()`.
//! This crate suggests `{}` as the convention for the map & set macros,
//! it matches their `Debug` output.
//!
//! Generic container macros already exist elsewhere, so those are not provided
//! here at the moment.

#[macro_export]
/// Create a **HashMap** from a list of key-value pairs
///
/// ## Example
///
/// ```
/// #[macro_use]
/// extern crate maplit;
/// # fn main() {
///
/// let map = hashmap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
macro_rules! hashmap {
    (@count) => (0);
    (@count $a:tt, $($rest:tt,)*) => (1 + hashmap!(@count $($rest,)*));
    
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (hashmap!($($key => $value),+));
    
    ( $($key:expr => $value:expr),* ) => {
        {
            let _cap = hashmap!(@count $($key,)*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                _map.insert($key, $value);
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
/// #[macro_use]
/// extern crate maplit;
/// # fn main() {
///
/// let set = hashset!{"a", "b"};
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
#[macro_export]
macro_rules! hashset {
    (@count) => (0);
    (@count $a:tt, $($rest:tt,)*) => (1 + hashset!(@count $($rest,)*));
    
    // trailing comma case
    ($($key:expr,)+) => (hashset!($($key),+));
    
    ( $($key:expr),* ) => {
        {
            let _cap = hashset!(@count $($key,)*);
            let mut _set = ::std::collections::HashSet::with_capacity(_cap);
            $(
                _set.insert($key);
            )*
            _set
        }
    };
}

#[macro_export]
/// Create a **BTreeMap** from a list of key-value pairs
///
/// ## Example
///
/// ```
/// #[macro_use]
/// extern crate maplit;
/// # fn main() {
///
/// let map = btreemap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// # }
/// ```
macro_rules! btreemap {
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (btreemap!($($key => $value),+));
    
    ( $($key:expr => $value:expr),* ) => {
        {
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}

#[macro_export]
/// Create a **BTreeSet** from a list of elements.
///
/// ## Example
///
/// ```
/// #[macro_use]
/// extern crate maplit;
/// # fn main() {
///
/// let set = btreeset!{"a", "b"};
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// # }
/// ```
macro_rules! btreeset {
    ($($key:expr,)+) => (btreeset!($($key),+));
    
    ( $($key:expr),* ) => {
        {
            let mut _set = ::std::collections::BTreeSet::new();
            $(
                _set.insert($key);
            )*
            _set
        }
    };
}

#[test]
fn test_hashmap() {
    use std::collections::HashMap;
    let names = hashmap!{
        1 => "one",
        2 => "two",
    };
    assert_eq!(names.len(), 2);
    assert_eq!(names[&1], "one");
    assert_eq!(names[&2], "two");
    assert_eq!(names.get(&3), None);
    
    let empty: HashMap<i32, i32> = hashmap!{};
    assert_eq!(empty.len(), 0);

    let _nested_compiles = hashmap!{
        1 => hashmap!{0 => 1 + 2,},
        2 => hashmap!{1 => 1,},
    };
}

#[test]
fn test_btreemap() {
    use std::collections::BTreeMap;
    let names = btreemap!{
        1 => "one",
        2 => "two",
    };
    assert_eq!(names.len(), 2);
    assert_eq!(names[&1], "one");
    assert_eq!(names[&2], "two");
    assert_eq!(names.get(&3), None);
    
    let empty: BTreeMap<i32, i32> = btreemap!{};
    assert_eq!(empty.len(), 0);

    let _nested_compiles = btreemap!{
        1 => btreemap!{0 => 1 + 2,},
        2 => btreemap!{1 => 1,},
    };
}
