use std::rc::Rc;

pub struct Params<'a> {
    pub title: &'a str,
    pub callbacks: Callbacks
}

#[derive(Clone)]
pub struct Callbacks {
    pub on_close: Rc<dyn Fn()->bool>
}