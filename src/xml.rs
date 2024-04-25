mod node;
use node::XmlNode;

#[derive(Debug)]
pub enum XmlError {
    Interfering(usize),
    NotClosed(usize),
}

#[derive(Debug, PartialEq)]
enum State {
    None,
    TagStart,
    TagEnd,
}

type Node = XmlNode;
impl TryFrom<&str> for Xml {
    type Error = XmlError;
    fn try_from(input:&str) -> Result<Self, Self::Error> {
        let mut tmp_nodes: Vec<(usize,Node)> = vec![];
        let mut current_name = String::new();
        let mut state = State::None;
        let mut items: Vec<Node> = vec![];
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
                        let node = match tmp_nodes.pop() {
                            Some((_,node)) => node,
                            None => Node::new(current_name.clone())
                        };
                        if node.name != current_name {
                            return Err(Self::Error::Interfering(i))
                        }
                        if let Some((_,parent_node)) = tmp_nodes.last_mut() {
                            parent_node.push(node)
                        } else {
                            items.push(node)
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
        println!("{:?}", items);
        Ok(Self {
            items
        })
    }
}

#[derive(Debug)]
pub struct Xml {
    items: Vec<XmlNode>
}
