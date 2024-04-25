use std::io::{BufReader, BufWriter, Read, Write};
mod node;
use node::XmlNode;

#[derive(Debug, PartialEq)]
pub enum XmlError {
    NotClosed(usize),
}

#[derive(Debug, PartialEq)]
enum State {
    None,
    TagStart,
    TagEnd,
}

type Node = XmlNode;

#[derive(Debug, PartialEq)]
pub struct Xml {
    items: Vec<Node>
}

impl TryFrom<&str> for Xml {
    type Error = XmlError;
    fn try_from(input:&str) -> Result<Self, Self::Error> {
        let mut tmp_nodes: Vec<(usize,Node)> = vec![];
        let mut current_name = String::new();
        let mut state = State::None;
        let mut xml = Xml::new();
        for (i, ch) in input.chars().enumerate() {
            match state {
                State::None => {
                    if ch == '<' {
                        state = State::TagStart;
                    }
                }
                State::TagStart => {
                    if ch == '/' {
                        state = State::TagEnd;
                    }
                    else if ch == '>' {
                        tmp_nodes.push((i,Node::new(current_name)));
                        current_name = String::new();
                        state = State::None;
                    } else {
                        current_name.push(ch);
                    }
                }
                State::TagEnd => {
                    if ch == '>' {
                        let (index, node) = match tmp_nodes.pop() {
                            Some((index,node)) => (index, node),
                            None => (i, Node::new(current_name.clone()))
                        };
                        if node.name != current_name {
                            return Err(Self::Error::NotClosed(index))
                        }
                        if let Some((_,parent_node)) = tmp_nodes.last_mut() {
                            parent_node.push(node)
                        } else {
                            xml.push(node)
                        }
                        current_name = String::new();
                        state = State::None;
                    } else {
                        current_name.push(ch);
                    }
                }
            }

        }
        if let Some((index, _)) = tmp_nodes.pop() {
            return Err(Self::Error::NotClosed(index))
        }
        println!("{:?}", xml);
        Ok(xml)
    }
}

impl Xml {
    pub fn new() -> Self {
        Self {
            items: vec![],
        }
    }

    pub(self) fn root(&self) -> Option<&Node> {
        if self.items.len() == 1 {
            self.items.first()
        } else {
            None
        }
    }

    pub(self) fn push(&mut self, value: Node) {
        self.items.push(value)
    }

    pub fn to_string(&self) -> String {
        self.items.iter().map(|item| item.to_string(0)).collect()
    }

    pub fn write<W: Write>(writer: &mut BufWriter<W>) {
        unimplemented!()
    }
}
mod tests {
    use super::*;
    #[test]
    fn test_simple_node() {
        let list = Xml::try_from("<a></a>").unwrap();
        let expected = Xml{items: vec![Node::new("a")]};
        assert_eq!(list, expected)
    }

    #[test]
    fn test_self_closing_node() {
        let list = Xml::try_from("<a/>").unwrap();
        let expected = Xml{items: vec![Node::new("a")]};
        assert_eq!(list, expected)
    }

    #[test]
    fn test_complex_node() {
        let list = Xml::try_from("<a><b><d></d></b><c><e></e></c></a><z></z>").unwrap();
        let a_node = Node {
            name: "a".to_string(),
            children: vec![
                Node {
                    name: "b".to_string(),
                    children: vec![Node::new("d")]
                },
                Node {
                    name: "c".to_string(),
                    children: vec![Node::new("e")]
                },
            ]
        };
        let expected = Xml{items: vec![a_node, Node::new("z")]};
        assert_eq!(list, expected)
    }

    #[test]
    fn test_simple_not_closed() {
        let err = Xml::try_from("<a>");
        if let Err(err) = err {

            assert_eq!(err, XmlError::NotClosed(2))
        }
    }

    #[test]
    fn test_node_after_not_closed() {
        let err = Xml::try_from("<a><p></p>");
        if let Err(err) = err {

            assert_eq!(err, XmlError::NotClosed(2))
        }
    }

    #[test]
    fn test_self_closing_after_not_closed() {
        let err = Xml::try_from("<a><p/>");
        if let Err(err) = err {

            assert_eq!(err, XmlError::NotClosed(2))
        }
    }

    #[test]
    fn test_complex_not_closed() {
        let err = Xml::try_from("<a><b><d></d></b><c><e></e></c></a><z><p/>");
        if let Err(err) = err {

            assert_eq!(err, XmlError::NotClosed(37))
        }
    }
}
