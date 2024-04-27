use std::io::{self, Read};
mod xml;
use xml::Xml;
use std::env;

fn main() -> io::Result<()>{
    let mut handle = io::stdin().lock();
    let mut input = String::new();
    handle.read_to_string(&mut input)?;
    let args: Vec<String> = env::args().collect();

    let xml = Xml::try_from(input.as_str());
    if let Err(err) = xml {
        println!("Error! {:?}", err)
    } else if let Ok(xml) = xml{
        let xml = if args.len() >= 2 {
            xml.search_query(&args[1])
        } else {
            xml
        };
        print!("{}", xml.to_string());
    }
    Ok(())
}
