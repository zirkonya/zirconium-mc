use clap::Parser;
use zmc::network::{
    context::{Bound, ClientContext, State},
    packet::Bundle,
    payload::Payload,
};
use zr_protocol::serialization::Serialize;
#[derive(clap::clap_derive::Parser)]
struct Args {
    #[clap(long)]
    state: State,
    #[clap(long)]
    bound: Bound,
    #[clap(long)]
    data: String,
    #[clap(long)]
    compression: Option<i32>,
}

fn main() {
    let Args {
        data,
        state,
        bound,
        compression,
    } = Args::parse();
    let bundle = Bundle::from_bytes(
        &data
            .as_bytes()
            .chunks(2)
            .map(|bytes| u8::from_str_radix(String::from_utf8_lossy(bytes).as_ref(), 16).unwrap())
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let packets = bundle.into_packets(&ClientContext {
        state,
        compression_set: compression.is_some(),
        compression_threshold: compression,
    });
    for packet in packets {
        match packet {
            Ok(mut packet) => {
                packet.bound = Some(bound);
                match Payload::from_packet(&packet) {
                    #[cfg(debug_assertions)]
                    Ok(Payload::NotYetImplemented(_)) => {
                        println!(
                            "NotYetImplemented (ID={:02x}; LEN={})",
                            packet.id(),
                            packet.payload().len()
                        );
                    }
                    Ok(Payload::ChunkDataAndUpdateLight(_)) => println!("ChunkData"),
                    Ok(payload) => println!("{:?}", payload),
                    Err(err) => eprintln!("{:02x} {err:?}", packet.id()),
                };
            }
            Err(err) => eprintln!("packet error : {err:?}"),
        }
    }
}
