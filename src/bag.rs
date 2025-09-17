pub trait Bag
where
    Self: Clone + Send + Sync + 'static,
{
    fn has(piece: char) -> bool;
    fn wildcard() -> Vec<char>;
}

#[derive(Clone, Debug)]
pub struct Bag7;
impl Bag for Bag7 {
    fn has(piece: char) -> bool {
        matches!(piece, 'I' | 'J' | 'O' | 'L' | 'Z' | 'S' | 'T')
    }

    fn wildcard() -> Vec<char> {
        vec!['T', 'I', 'L', 'J', 'O', 'S', 'Z']
    }
}
