#[derive(Debug, PartialEq)]
pub (super) struct XmlNode {
    pub(super) name: String,
    pub(super) children: Vec<XmlNode>,
}

const TAB:&str = "   ";
impl XmlNode {
    pub fn new<S>(name: S) -> Self where
    S: ToString {
        XmlNode {
            name: name.to_string(),
            children: vec![]
        }
    }

    pub fn push(&mut self,item: Self) {
        self.children.push(item)
    }

    pub fn to_string(&self, depth: usize) -> String {
        let mut inner = String::new();
        for child in &self.children {
            inner.push_str(child.to_string(depth+1).as_str())
        }
        format!("{}<{}>\n{}{}</{}>\n", TAB.repeat(depth),self.name, inner,TAB.repeat(depth), self.name)
    }
}
