use std::any::{Any, TypeId};

macro_rules! std_instance_of {
    ($label: ty) => {
        impl InstanceOf for $label {}
    };
}

pub trait InstanceOf 
where
    Self: Any
{
    fn instance_of<U: ?Sized + Any>(&self) -> bool {
        TypeId::of::<Self>() == TypeId::of::<U>()
    }
}

std_instance_of!(bool);
std_instance_of!(i8);
std_instance_of!(u8);
std_instance_of!(i16);
std_instance_of!(u16);
std_instance_of!(i32);
std_instance_of!(u32);
std_instance_of!(i64);
std_instance_of!(u64);
std_instance_of!(i128);
std_instance_of!(u128);
std_instance_of!(f32);
std_instance_of!(f64);
std_instance_of!(String);

impl <T: InstanceOf> InstanceOf for Vec<T> {}