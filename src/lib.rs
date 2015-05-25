#![warn(missing_docs)]

//! Macros for container literals with specific type.
//! 
//! ```
//! #[macro_use]
//! extern crate maplit;
//!
//! # fn main() {
//! let foo = hashmap!{
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
/// let foo = hashmap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(foo["a"], 1);
/// assert_eq!(foo["b"], 2);
/// assert_eq!(foo.get("c"), None);
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
/// let foo = hashset!{"a", "b"};
/// assert!(foo.contains("a"));
/// assert!(foo.contains("b"));
/// assert!(!foo.contains("c"));
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
/// let foo = btreemap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(foo["a"], 1);
/// assert_eq!(foo["b"], 2);
/// assert_eq!(foo.get("c"), None);
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
/// let foo = btreeset!{"a", "b"};
/// assert!(foo.contains("a"));
/// assert!(foo.contains("b"));
/// assert!(!foo.contains("c"));
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
