use std::rc::Rc;
pub use purust_core::SharedRecord as STObject;

fn purust_object_st_action(
    action: impl Fn() -> crate::UnknownType + 'static,
) -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(move |_| action())))
}

fn purust_object_st_handle(object: Rc<STObject>) -> crate::UnknownType {
    // Generated consumers downcast the native Rc<STObject>, not STObject itself.
    crate::Value::Class(Rc::new(object))
}

pub fn Foreign_Object_ST_new() -> crate::UnknownType {
    purust_object_st_action(|| purust_object_st_handle(Rc::new(STObject::empty())))
}

pub fn Foreign_Object_ST_poke(
    key: String,
    value: crate::UnknownType,
    object: Rc<STObject>,
) -> crate::UnknownType {
    purust_object_st_action(move || {
        let previous = object.insert(key.clone(), value.clone());
        drop(previous);
        purust_object_st_handle(object.clone())
    })
}

pub fn Foreign_Object_ST_delete(key: String, object: Rc<STObject>) -> crate::UnknownType {
    purust_object_st_action(move || {
        let previous = object.remove(&key);
        drop(previous);
        purust_object_st_handle(object.clone())
    })
}

pub fn Foreign_Object_ST_peekImpl(
    just: purust_core::Func1<crate::UnknownType, crate::UnknownType>,
    nothing: crate::UnknownType,
    key: String,
    object: Rc<STObject>,
) -> crate::UnknownType {
    purust_object_st_action(move || {
        // The callback may access this same object: release the lock first.
        let value = object.get(&key);
        match value {
            Some(value) => just(value),
            None => nothing.clone(),
        }
    })
}
