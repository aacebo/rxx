use std::{marker::PhantomData, rc::Rc};

pub struct Stream<In, Out>
where
    In: 'static,
    Out: 'static,
{
    __in__: PhantomData<In>,
    __out__: PhantomData<Out>,
    pub(crate) action: Rc<dyn Fn(&In) -> Out>,
}

impl<In, Out> Stream<In, Out>
where
    In: 'static,
    Out: 'static,
{
    pub fn new(action: impl Fn(&In) -> Out + 'static) -> Self {
        return Self {
            __in__: PhantomData,
            __out__: PhantomData,
            action: Rc::new(action),
        };
    }

    pub fn map<Next>(self, action: impl Fn(&Out) -> Next + 'static) -> Stream<In, Next> {
        return Stream::new(move |input| {
            let out = (self.action)(input);
            return (action)(&out);
        });
    }

    pub fn run(&self, input: &In) -> Out {
        return (self.action)(input);
    }
}
