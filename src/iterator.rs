use crate::Stream;

pub trait IterStream<Item> {
    fn filter<P>(&self, predicate: P) -> Self
    where
        P: Fn(&Item) -> bool + 'static;
}

impl<Item> IterStream<Item> for Stream<&[Item], Vec<Item>>
where
    Self: IntoIterator<Item = Item>,
    Item: Clone,
{
    fn filter<P>(&self, predicate: P) -> Self
    where
        P: Fn(&Item) -> bool + 'static,
    {
        return Stream::<&[Item], Vec<Item>>::new(move |input| {
            let mut out = vec![];

            for item in input.iter() {
                if !(predicate)(item) {
                    out.push(item.clone());
                }
            }

            return out;
        });
    }
}
