// Automatically generated rust module for 'dir.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct File {
    pub name: String,
    pub contents: Vec<u8>,
}

impl<'a> MessageRead<'a> for File {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.contents = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for File {
    fn get_size(&self) -> usize {
        0
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.contents.is_empty() { 0 } else { 1 + sizeof_len((&self.contents).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if !self.contents.is_empty() { w.write_with_tag(18, |w| w.write_bytes(&**&self.contents))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Directory {
    pub name: String,
    pub contents: Vec<Entity>,
}

impl<'a> MessageRead<'a> for Directory {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.contents.push(r.read_message::<Entity>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Directory {
    fn get_size(&self) -> usize {
        0
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + self.contents.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        for s in &self.contents { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Entity {
    pub entity: mod_Entity::OneOfentity,
}

impl<'a> MessageRead<'a> for Entity {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.entity = mod_Entity::OneOfentity::directory(r.read_message::<Directory>(bytes)?),
                Ok(18) => msg.entity = mod_Entity::OneOfentity::file(r.read_message::<File>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Entity {
    fn get_size(&self) -> usize {
        0
        + match self.entity {
            mod_Entity::OneOfentity::directory(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Entity::OneOfentity::file(ref m) => 1 + sizeof_len((m).get_size()),
            mod_Entity::OneOfentity::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.entity {            mod_Entity::OneOfentity::directory(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_Entity::OneOfentity::file(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_Entity::OneOfentity::None => {},
    }        Ok(())
    }
}

pub mod mod_Entity {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfentity {
    directory(Directory),
    file(File),
    None,
}

impl Default for OneOfentity {
    fn default() -> Self {
        OneOfentity::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Top {
    pub tree: Vec<u64>,
    pub ent: Vec<u8>,
}

impl<'a> MessageRead<'a> for Top {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.tree = r.read_packed(bytes, |r, bytes| Ok(r.read_uint64(bytes)?))?,
                Ok(18) => msg.ent = r.read_bytes(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Top {
    fn get_size(&self) -> usize {
        0
        + if self.tree.is_empty() { 0 } else { 1 + sizeof_len(self.tree.iter().map(|s| sizeof_varint(*(s) as u64)).sum::<usize>()) }
        + if self.ent.is_empty() { 0 } else { 1 + sizeof_len((&self.ent).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_packed_with_tag(10, &self.tree, |w, m| w.write_uint64(*m), &|m| sizeof_varint(*(m) as u64))?;
        if !self.ent.is_empty() { w.write_with_tag(18, |w| w.write_bytes(&**&self.ent))?; }
        Ok(())
    }
}

