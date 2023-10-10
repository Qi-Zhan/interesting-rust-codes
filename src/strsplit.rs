//! A string splitting iterator for learning lifetimes.
//!
//! Copy from <https://www.youtube.com/watch?v=rAl-9HwD858>

/// A string splitting iterator.
///
/// lifetimes:
/// - `'haystack` is the lifetime of the string we are splitting.
/// - `'delim` is the lifetime of the delimiter.
#[derive(Debug)]
pub struct StrSplit<'haystack, 'delim> {
    remainder: Option<&'haystack str>,
    delimiter: &'delim str,
}

impl<'haystack, 'delim> StrSplit<'haystack, 'delim> {
    pub fn new(haystack: &'haystack str, delimiter: &'delim str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

/// Can write as `impl<'haystack> Iterator for StrSplit<'haystack, '_>`.
impl<'haystack, 'delim> Iterator for StrSplit<'haystack, 'delim> {
    /// Lifetime of the returned item is the same as the lifetime of the haystack.
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delim_start, delim_end)) = remainder
            .find(self.delimiter)
            .map(|start| (start, start + self.delimiter.len()))
        {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

#[test]
fn until_char_test() {
    let delim = "c".to_string();
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, &delim).collect();
    assert_eq!(letters, vec!["a b ", " d e"]);
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
