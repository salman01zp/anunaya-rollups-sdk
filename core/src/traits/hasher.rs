use std::fmt::Debug;

pub trait HasherT: Sync + Send {
    type Output: AsRef<[u8]> + Send + Sync + Clone + Debug + Copy;
    fn hash(s: &[u8]) -> Self::Output;
}