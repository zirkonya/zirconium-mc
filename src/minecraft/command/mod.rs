use std::str::FromStr;
use crate::{format::text::identifier::Identifier, gen_bin, gen_struct, tools::{maths::{varint::varint::VarInt, vector::{vector2::Vector2f, vector3::{Location, Vector3f}}}, utils::bin::binarray::Array}};

// const NODE_TYPE: u8 = 0x03;
// const IS_EXECUTABLE: u8 = 0x04;
// const HAS_REDIRECT: u8 = 0x08;
// const HAS_SUGGESTION_TYPE: u8 = 0x10;

gen_bin!(Node {
    flags: u8,
    children_count: VarInt<i32>,
    children: Array<VarInt<i32>>,
    redirect_node: Option<VarInt<i32>>,
    name: String,
    parser_id: Option<VarInt<i32>>,
    properties: Option<()>,
    suggestion_type: Option<Identifier>
});

pub trait Parser<T: FromStr> {
    fn parse(s: &str) -> Option<T>;
}

macro_rules! parser {
    ($parser: ident -> $t:ty) => {
        pub struct $parser;

        impl Parser<$t> for $parser {
            fn parse(s: &str) -> Option<$t> {
                s.parse().ok()
            }
        }
    };
}

parser!(BrigadierBool -> bool);
parser!(BrigadierFloat -> f32);
parser!(BrigadierDouble -> f64);
parser!(BrigadierInteger -> i32);
parser!(BrigadierLong -> i64);
parser!(BrigadierString -> String);

parser!(MinecraftBlockPos -> Location);
parser!(MinecraftVec3 -> Vector3f);
parser!(MinecraftVec2 -> Vector2f);