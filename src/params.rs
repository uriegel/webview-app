use std::rc::Rc;

#[derive(Clone)]
pub struct Params<'a> {
    pub title: &'a str,
    pub on_close: Rc<dyn Fn()->bool>
}
