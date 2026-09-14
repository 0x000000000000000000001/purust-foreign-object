use std::rc::Rc;

// Both sides of _copyST use the same native type and Value handle boxing.
pub use Purs_Foreign_Object_ST::STObject as Object;

fn purust_object_function(
    function: impl Fn(crate::UnknownType) -> crate::UnknownType + 'static,
) -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(function)))
}

pub fn Foreign_Object_empty() -> Rc<Object> {
    Rc::new(Object::empty())
}

pub fn Foreign_Object__copyST(value: crate::UnknownType) -> crate::UnknownType {
    let object = value.unwrap_class::<Rc<Object>>().clone();
    purust_object_function(move |_| {
        // Read and copy at action execution, with a fresh map on every replay.
        crate::Value::Class(Rc::new(Rc::new(object.snapshot())))
    })
}

pub fn Foreign_Object_runST(action: crate::UnknownType) -> Rc<Object> {
    action.unwrap_func1()(crate::Value::Unit)
        .unwrap_class::<Rc<Object>>()
        .clone()
}

pub fn Foreign_Object__lookup() -> crate::UnknownType {
    // The current generated ABI curries Fn4 as four Value functions.
    // This native map reads own entries; it does not emulate JS prototypes.
    purust_object_function(move |nothing| {
        purust_object_function(move |just| {
            let nothing = nothing.clone();
            purust_object_function(move |key| {
                let nothing = nothing.clone();
                let just = just.clone();
                let key = key.unwrap_string();
                purust_object_function(move |object| {
                    // get clones the payload and releases the lock before just.
                    match object.unwrap_class::<Rc<Object>>().get(&key) {
                        Some(value) => just.unwrap_func1()(value),
                        None => nothing.clone(),
                    }
                })
            })
        })
    })
}

pub fn Foreign_Object__mapWithKey() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Shared(Rc::new(move |object, callback| {
        let entries = object.unwrap_class::<Rc<Object>>().entries();
        let mapped = entries.into_iter().map(|(key, value)| {
            let result = callback.unwrap_func1()(crate::Value::String(key.clone()))
                .unwrap_func1()(value);
            (key, result)
        }).collect();
        crate::Value::Class(Rc::new(Rc::new(Object::from_entries(mapped))))
    })))
}

pub fn Foreign_Object_toArrayWithKey(
    callback: purust_core::Func2<String, crate::UnknownType, crate::UnknownType>,
    object: Rc<Object>,
) -> crate::UnknownType {
    // SharedRecord enumerates own keys in JS order. Keep the initial keys, but
    // read each value just before its callback: callbacks may update/delete a
    // later entry. Never hold a record lock while invoking user code.
    let keys: Vec<String> = object.entries().into_iter().map(|(key, _)| key).collect();
    let mut result = Vec::with_capacity(keys.len());
    for key in keys {
        if let Some(value) = object.get(&key) {
            result.push(callback(key, value));
        }
    }
    purust_core::mk_array(result)
}
