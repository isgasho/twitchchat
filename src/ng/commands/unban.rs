use crate::ng::Encodable;
use std::io::{Result, Write};

use super::ByteWriter;

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct Unban<'a> {
    pub channel: &'a str,
    pub username: &'a str,
}

pub fn unban<'a>(channel: &'a str, username: &'a str) -> Unban<'a> {
    Unban { channel, username }
}

impl<'a> Encodable for Unban<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).command(self.channel, &[&"/unban", &self.username])
    }
}
