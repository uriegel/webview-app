use std::rc::Rc;

#[derive(Clone)]
pub struct Callbacks {
    pub on_close: Rc<dyn Fn()->bool>
}
