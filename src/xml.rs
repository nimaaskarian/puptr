use std::{collections::HashMap, default, io::{BufReader, BufWriter, Read, Write}};
mod node;
use node::XmlNode;

#[derive(Debug, PartialEq)]
pub enum XmlError {
    NotClosed(usize),
}

#[derive(Debug, PartialEq, Default)]
enum State {
    #[default]
    None,
    TagStart,
    AttributeName,
    AttributeValue,
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
        let mut nodes: Vec<(usize,Node)> = vec![];
        let mut current_name = String::new();
        let mut current_text = String::new();
        let mut attribute_name = String::new();
        let mut attribute_value = String::new();
        let mut current_attributes: HashMap<String, String> = HashMap::new();
        let mut state = Default::default();
        let mut xml = Xml::new(vec![]);
        for (i, ch) in input.chars().enumerate() {
            match state {
                State::None => {
                    match ch {
                        '<' => {
                            if !current_text.is_empty() {
                                let (_, node) = nodes.last_mut().unwrap();
                                node.push(Node::new_text(current_text.trim_end()));
                                current_text = String::new();
                            }
                            state = State::TagStart;
                        }
                        any if !any.is_whitespace() || !current_text.is_empty() => {
                            current_text.push(ch)
                        }
                        _ => {}
                    }
                }
                State::TagStart => {
                    match ch {
                        '/' => {
                            state = State::TagEnd;
                        }
                        '>' => {
                            nodes.push((i, Node::new_with_attributes(current_name, current_attributes)));
                            current_name = String::new();
                            current_attributes = HashMap::new();
                            state = Default::default();
                        }
                        any if any.is_whitespace() => {
                            let next_char = input.chars().nth(i+1).unwrap();
                            if next_char != '>' && !next_char.is_whitespace() {
                                state = State::AttributeName;
                            }
                        }
                        any => {
                            current_name.push(any);
                        }
                    }
                }
                State::AttributeName => {
                    if ch == '"' && input.chars().nth(i-1).unwrap() == '='{
                        state = State::AttributeValue;
                        attribute_name.pop();
                    } else {
                        attribute_name.push(ch);
                    }
                }
                State::AttributeValue => {
                    match ch {
                        '"' => {
                            state = State::TagStart;
                            if !attribute_value.is_empty() && !attribute_name.is_empty() {
                                current_attributes.insert(attribute_name, attribute_value);
                                attribute_value = String::new();
                                attribute_name = String::new();
                            }
                        }
                        any => {
                            attribute_value.push(any);
                        }
                    }
                }
                State::TagEnd => {
                    match ch {
                        '>' => {
                            let (index, node) = match nodes.pop() {
                                Some((index,node)) => (index, node),
                                None => (i, Node::new(current_name.clone()))
                            };
                            if node.name != current_name {
                                return Err(Self::Error::NotClosed(index))
                            }
                            if let Some((_, parent_node)) = nodes.last_mut() {
                                parent_node.push(node)
                            } else {
                                xml.push(node)
                            }
                            current_name = String::new();
                            state = Default::default();
                        }
                        any => {
                            current_name.push(any);
                        }
                    }
                }
            }
        }
        if let Some((index, _)) = nodes.pop() {
            return Err(Self::Error::NotClosed(index))
        }
        Ok(xml)
    }
}

impl Xml {
    pub fn new(items: Vec<XmlNode>) -> Self {
        Self {
            items,
        }
    }

    pub fn root(&self) -> Option<&Node> {
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

    pub fn search_query<S>(&self, query: S)  -> Xml where S: ToString {
        let query = query.to_string();
        let mut chars = query.chars().into_iter();
        if let Some(char) = chars.next() {
            if char == '.' {
                let class_name: String = chars.collect();
                return self.root().unwrap().search(&|node| node.has_class(class_name.clone()))
            } else {
                chars.next_back();
                let node_name: String = chars.collect();
                let node_name = format!("{char}{node_name}");
                return self.root().unwrap().search(&|node| node.name == node_name)
            }
        }
        Xml::new(vec![])
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
        let a_node = Node::new_with_children("a",
            vec![
                    Node::new_with_children("b",
                        vec![Node::new("d")]
                    ),
                    Node::new_with_children("c",
                        vec![Node::new("e")]
                    ),
            ]
        );
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
