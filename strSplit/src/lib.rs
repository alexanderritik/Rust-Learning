//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

pub trait Delimiter {
    fn find_next(&self, s: &str)-> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn find_next(&self, s: &str)-> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start+ self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str)-> Option<(usize, usize)> {
        s.char_indices().find(|(_, c)| c == self).map(|(pos, start)| (pos , pos + 1))
    }
}


pub struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSplit<'a, D> {
    pub fn new(haystack: &'a str, delimiter: D) -> Self {
        Self{
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a, D> Iterator for StrSplit<'a, D> where  D: Delimiter{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remaninder) = self.remainder{
            if let Some((start, end)) = self.delimiter.find_next(&remaninder) {
                let until_delimiter = &remaninder[..start];
                *remaninder = &remaninder[end..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

fn until_chars(s: &str, c: char) -> &str {
    let delim = format!("{}",c);
    StrSplit::new(s, &*delim).next().expect("It will always return one result")
}

#[test]
fn until_chars_test(){
    let result = until_chars("hello World", 'o');
    assert_eq!(result, "hell");
}


#[test]
fn it_works(){
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert_eq!(letters.collect::<Vec<&str>>(), vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn it_works_corner_case(){
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert_eq!(letters.collect::<Vec<&str>>(), vec!["a", "b", "c", "d", ""]);
}