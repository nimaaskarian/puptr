mod xml;
use xml::Xml;

fn main() {
    let list = Xml::try_from("<a><b><d></d></b><c><e></e></c></a><z><p/>");
    if let Err(err) = list {
        println!("Error! {:?}", err)
    } else {
        list.unwrap().print()
    }
}
