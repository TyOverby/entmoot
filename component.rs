#![allow(dead_code)]
use sigl::Sigl;

pub trait Component {
    fn component_id(s: Sigl<Self>)-> uint;
}
