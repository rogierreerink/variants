use syn::{punctuated::Punctuated, token::Token};

pub trait VecExt<T> {
    fn into_punctuated<P: Token + Default>(self) -> Punctuated<T, P>;
}

impl<T> VecExt<T> for Vec<T>
where
    T: Clone,
{
    fn into_punctuated<P: Token + Default>(self) -> Punctuated<T, P> {
        let mut puncted = Punctuated::<_, P>::new();

        for item in self {
            puncted.push(item.clone());
        }

        puncted
    }
}
