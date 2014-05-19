#![allow(dead_code)]
use sigl::Sigl;

pub trait Component {
    fn id(s: Sigl<Self>)-> uint;
}
