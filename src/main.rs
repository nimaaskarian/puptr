mod xml;
use xml::Xml;

fn main() {
    let list = Xml::try_from("<a><b><d></d></b><c><e></e></c></a>");
    if let Err(err) = list {
        println!("Error! {:?}", err)
    } else {
        print!("{}", list.unwrap().to_string())
    }
}
