#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Optional<T>
{
    None, Some(T)
}

impl <T: Clone> Optional<T> {
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        match self {
            Self::Some(_) => false,
            Self::None => true,
        }
    }

    #[inline(always)]
    pub fn is_some(&self) -> bool {
        match self {
            Self::Some(_) => true,
            Self::None => false,
        }
    }

    pub fn unwrap(&self) -> T {
        match self {
            Optional::Some(t) => t.clone(),
            Optional::None => panic!("Optional is None"),
        }
    }

    
    pub fn unwrap_or(&self, default: T) -> T {
        match self {
            Optional::Some(t) => t.clone(),
            Optional::None => default,
        }
    }

    pub fn unwrap_or_else<F>(&self, f: F) -> T 
        where F: FnOnce() -> T {
        match self {
            Optional::Some(t) => t.clone(),
            Optional::None => f(),
        }
    }
}

impl <T: Default + Clone> Optional<T> {
    pub fn unwrap_or_default(&self) -> T {
        match self {
            Optional::Some(t) => t.clone(),
            Optional::None => T::default(),
        }
    }
}