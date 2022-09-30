/// Creates a HashMap from provided key/value pairs.
///
/// ## Example
/// ```
/// use std::collections::HashMap;
/// use hashmap_macro::hashmap;
///
/// let m: HashMap<&str, u32> = hashmap![];
/// let m: HashMap<&str, u32> = hashmap!("a" => 1);
/// let m: HashMap<&str, u32> = hashmap! {
///     "a" => 1,
///     "b" => 2,
/// };
/// ```
#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };

    ($($key:expr => $value:expr),+ $(,)?) => {
        ::std::collections::HashMap::from([ $(($key, $value)),* ])
    };
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn empty() {
        let m: HashMap<&str, u32> = hashmap![];
        assert!(m.is_empty());
    }

    #[test]
    fn one_pair() {
        let m: HashMap<&str, u32> = hashmap! {
            "a" => 1
        };
        assert!(!m.is_empty());
        assert_eq!(*m.get("a").unwrap(), 1);
    }

    #[test]
    fn two_pairs() {
        let m: HashMap<&str, u32> = hashmap! {
            "a" => 1,
            "b" => 2,
        };
        assert!(!m.is_empty());
        assert_eq!(*m.get("a").unwrap(), 1);
        assert_eq!(*m.get("b").unwrap(), 2);
    }
}
