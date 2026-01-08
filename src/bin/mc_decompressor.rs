use std::io::{self, Cursor, Read};
use zmc::network::{
    context::{ClientContext, State},
    packet::Bundle,
};
use zr_protocol::serialization::Serialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    let data = if args.len() > 1 {
        hex::decode(&args[1]).unwrap()
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)?;
        hex::decode(input.trim())?
    };
    let mut cursor = Cursor::new(data);
    let bundle = Bundle::read_from(&mut cursor).unwrap();
    for packet in bundle.into_packets(&ClientContext {
        state: State::Config,
        compression_set: true,
        compression_threshold: Some(256),
    }) {
        println!("{:02x?}", packet);
    }
    Ok(())
}
