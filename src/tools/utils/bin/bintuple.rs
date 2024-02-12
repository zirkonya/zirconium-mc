use crate::tools::utils::bin::*;

impl Binary for () {
    fn to_bin(&self) -> Vec<u8> {
        vec![]
    }
    
    fn from_bin(_: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        Ok(())
    }

    fn byte_length(&self) -> usize {
        0
    }
}

impl <A, B> Binary for (A, B)
    where
        A: Binary,
        B: Binary,
{
    fn to_bin(&self) -> Vec<u8> {
        let mut bin = Vec::new();

        bin.append(&mut self.0.to_bin());
        bin.append(&mut self.1.to_bin());

        bin
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        let cursor = 0_usize;
        let a = A::from_bin(bin[cursor..].to_vec())?;
        let cursor = cursor + a.byte_length();
        let b = B::from_bin(bin[cursor..].to_vec())?;
        Ok((a,b))
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        self.0.byte_length() + self.1.byte_length()
    }
}

impl <A, B, C> Binary for (A, B, C)
    where
        A: Binary,
        B: Binary,
        C: Binary
{
    fn to_bin(&self) -> Vec<u8> {
        let mut bin = Vec::new();

        bin.append(&mut self.0.to_bin());
        bin.append(&mut self.1.to_bin());
        bin.append(&mut self.2.to_bin());

        bin
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        let cursor = 0_usize;
        let a = A::from_bin(bin[cursor..].to_vec())?;
        let cursor = cursor + a.byte_length();
        let b = B::from_bin(bin[cursor..].to_vec())?;
        let cursor = cursor + b.byte_length();
        let c = C::from_bin(bin[cursor..].to_vec())?;
        Ok((a,b,c))
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        self.0.byte_length() + self.1.byte_length() + self.2.byte_length()
    }
}

impl <A, B, C, D> Binary for (A, B, C, D)
    where
        A: Binary,
        B: Binary,
        C: Binary,
        D: Binary
{
    fn to_bin(&self) -> Vec<u8> {
        let mut bin = Vec::new();

        bin.append(&mut self.0.to_bin());
        bin.append(&mut self.1.to_bin());
        bin.append(&mut self.2.to_bin());
        bin.append(&mut self.3.to_bin());

        bin
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        let cursor = 0_usize;
        let a = A::from_bin(bin[cursor..].to_vec())?;
        let cursor = cursor + a.byte_length();
        let b = B::from_bin(bin[cursor..].to_vec())?;
        let cursor = cursor + b.byte_length();
        let c = C::from_bin(bin[cursor..].to_vec())?;
        let cursor = cursor + c.byte_length();
        let d = D::from_bin(bin[cursor..].to_vec())?;
        Ok((a,b,c,d))
    }

    #[inline(always)]
    fn byte_length(&self) -> usize {
        self.0.byte_length() + self.1.byte_length() + self.2.byte_length() + self.3.byte_length()
    }
}