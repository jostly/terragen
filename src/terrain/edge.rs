#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Edge<V = u32> {
    pub a: V,
    pub b: V,
}

impl<V> Edge<V>
    where V: PartialOrd
{
    pub fn new(a: V, b: V) -> Edge<V> {
        if a <= b {
            Edge { a: a, b: b }
        } else {
            Edge { a: b, b: a }
        }
    }
}
