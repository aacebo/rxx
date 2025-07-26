pub type Transformer<In, Out> = dyn Fn(In) -> Out;

pub struct Stream<In, Out>
where
    In: 'static,
    Out: 'static,
{
    pub(crate) action: Box<Transformer<In, Out>>,
}

impl<In, Out> Stream<In, Out>
where
    In: 'static,
    Out: 'static,
{
    pub fn new<Action>(action: Action) -> Self
    where
        Action: Fn(In) -> Out + 'static,
    {
        return Self {
            action: Box::new(action),
        };
    }

    pub fn map<Next, Action>(self, action: Action) -> Stream<In, Next>
    where
        Action: Fn(Out) -> Next + 'static,
    {
        return Stream::new(move |input| {
            let out = (self.action)(input);
            return (action)(out);
        });
    }

    pub fn run(&self, input: In) -> Out {
        return (self.action)(input);
    }
}