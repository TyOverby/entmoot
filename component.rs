pub struct Sigl<T>;

pub trait Component {
    fn id(s: Sigl<Self>)-> uint;
}
