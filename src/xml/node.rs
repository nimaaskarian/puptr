use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub (super) struct XmlNode {
    pub(super) name: String,
    pub(super) text: String,
    pub(super) children: Vec<Self>,
    pub(super) attributes: HashMap<String, String>,
}
impl Default for XmlNode {
    fn default() -> Self {
        XmlNode {
            name: String::new(),
            children: vec![],
            attributes: HashMap::new(),
            text: String::new(),
        }
    }
}

const TAB:&str = "   ";
impl XmlNode {
    pub fn new<S>(name: S) -> Self where
    S: ToString {
        XmlNode {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn new_text<S>(text: S) -> Self where S: ToString {
        XmlNode {
            text: text.to_string(),
            ..Default::default()
        }
    }

    pub fn new_with_children<S>(name: S, children: Vec<Self>)  -> Self where
    S:ToString {
        XmlNode {
            children,
            ..Self::new(name)
        }
    }

    pub fn new_with_attributes<S>(name: S, attributes: HashMap<String, String>) -> Self where
    S: ToString {
        XmlNode {
            attributes,
            ..Self::new(name)
        }
    }

    pub fn push(&mut self,item: Self) {
        self.children.push(item)
    }

    pub fn to_string(&self, depth: usize) -> String {
        let inner: String = self.children.iter()
            .map(|child| child.to_string(depth+1))
            .collect();
        let attributes: String = self.attributes.iter()
            .map(|(key, value)| format!(" {key}=\"{value}\""))
            .collect();
        let tabs = TAB.repeat(depth);
        let name = &self.name;
        if name.is_empty() {
            format!("{tabs}{}\n", self.text)
        } else {
            format!("{tabs}<{name}{attributes}>\n{inner}{tabs}</{name}>\n")
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let node = XmlNode::new_with_children("html", vec![XmlNode::new("a")]);
        let expected = format!("<html>\n{TAB}<a>\n{TAB}</a>\n</html>\n");
        assert_eq!(expected, node.to_string(0))
    }
}
