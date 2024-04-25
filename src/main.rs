use std::io::{self, Read};
mod xml;
use xml::Xml;

fn main() -> io::Result<()>{
    let mut handle = io::stdin().lock();
    let mut input = String::new();
    handle.read_to_string(&mut input)?;

    let list = Xml::try_from(input.as_str());
    if let Err(err) = list {
        println!("Error! {:?}", err)
    } else {
        print!("{}", list.unwrap().to_string())
    }
    Ok(())
}
