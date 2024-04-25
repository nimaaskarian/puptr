use std::io::{BufReader, BufWriter, Read, Write};

#[derive(Debug)]
pub(super) struct XmlNode {
    pub(super) name: String,
    pub(super) children: Vec<XmlNode>,
}

impl XmlNode {
    pub fn new(name: String) -> Self {
        XmlNode {
            name,
            children: vec![]
        }
    }

    pub fn default() -> Self {
        Self::new(String::from("a"))
    }

    pub fn push(&mut self,item: Self) {
        self.children.push(item)
    }

    pub fn write<W: Write>(writer: &mut BufWriter<W>) {
        unimplemented!()
    }

    pub fn reader<R: Read>(reader: &mut BufReader<R>) {
        unimplemented!()
    }
}
