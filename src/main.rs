mod xml;
use xml::Xml;

fn main() {
    let root = Xml::try_from("<a><b><d></d></b><c><e></e></c></a><z><p/>");
    if let Err(err) = root {
        println!("Error! {:?}", err)
    }
}
