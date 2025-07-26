use crate::Stream;

pub type Predicate<Item> = dyn Fn(Item) -> bool;

pub trait IterStream<Item> {
    fn filter<P>(&self, predicate: P) -> Self where P: Fn(Item) -> bool;
}

impl<Item> IterStream<Item> for Stream<&[Item], Vec<Item>>
where
    Self: IntoIterator<Item = Item>,
{
    fn filter<P>(&self, predicate: P) -> Self
    where
        P: Fn(Item) -> bool,
    {
        return Stream::new(move |input| {
            let mut out = (self.action)(input.clone());

            for item in out.into_iter() {
                if !(predicate)(item) {
                    // remove item
                }
            }

            return out;
        });
    }
}