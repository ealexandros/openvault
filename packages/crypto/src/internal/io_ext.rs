use std::io::{Read, Seek, Write};

pub trait ReadSeek: Read + Seek {}

pub trait WriteSeek: Write + Seek {}

impl<R: Read + Seek> ReadSeek for R {}

impl<W: Write + Seek> WriteSeek for W {}

pub type Writer = dyn WriteSeek;

pub type Reader = dyn ReadSeek;
