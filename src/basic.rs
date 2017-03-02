use std::{io, fs};
use std::io::prelude::*;
use std::path::Path;
use super::combine::{self, Parser, State};

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum RawPart<S> {
    Tag(S, Vec<RawField<S>>),
    Comment(S),
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct RawField<S: ?Sized>(pub S);

pub fn parse<S>(raw_contents: S) -> Vec<RawPart<String>>
    where S: Into<String>
{
    parse_impl(raw_contents.into())
}

fn parse_impl(raw_contents: String) -> Vec<RawPart<String>> {
    unimplemented!()
}

pub fn parse_ref<'a, S>(raw_contents: &'a S) -> Vec<RawPart<&'a str>>
    where S: AsRef<str>
{
    parse_ref_impl(raw_contents.as_ref())
}

fn parse_ref_impl<'a>(raw_contents: &'a str) -> Vec<RawPart<&'a str>> {
    unimplemented!()
}

pub fn parse_mut<'a, S>(raw_contents: &'a mut S) -> Vec<RawPart<&'a mut str>>
    where S: AsMut<str>
{
    parse_mut_impl(raw_contents.as_mut())
}

fn parse_mut_impl<'a>(raw_contents: &'a mut str) -> Vec<RawPart<&'a mut str>> {
    unimplemented!()
}

pub fn parse_from<'a, S>(raw_file_path: &'a S) -> io::Result<Vec<RawPart<String>>>
    where S: AsRef<Path>
{
    parse_from_impl(raw_file_path.as_ref())
}

fn parse_from_impl<'a>(raw_file_path: &'a Path) -> io::Result<Vec<RawPart<String>>> {
    let mut strbuf = String::new();
    io::BufReader::new(fs::File::open(raw_file_path)?).read_to_string(&mut strbuf)?;
    Ok(parse_impl(strbuf))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {}
}
