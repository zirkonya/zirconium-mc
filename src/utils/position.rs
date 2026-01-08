use zr_protocol::serialization::Serialize;

pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Serialize for Position {
    fn write_to<W: std::io::Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        todo!()
    }

    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized {
        todo!()
    }
}
