use std::io::{self, Read};
mod xml;
use xml::Xml;

fn main() -> io::Result<()>{
    let mut handle = io::stdin().lock();
    let mut input = String::new();
    handle.read_to_string(&mut input)?;

    let xml = Xml::try_from(input.as_str());
    if let Err(err) = xml {
        println!("Error! {:?}", err)
    } else if let Ok(xml) = xml{
        print!("{}", xml.to_string());
    }
    Ok(())
}
