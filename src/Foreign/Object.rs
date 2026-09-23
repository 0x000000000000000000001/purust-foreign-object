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

pub fn Foreign_Object__foldM(
    bind: purust_core::Func2<
        crate::UnknownType,
        purust_core::Func1<crate::UnknownType, crate::UnknownType>,
        crate::UnknownType,
    >,
    callback: purust_core::Func3<
        crate::UnknownType,
        String,
        crate::UnknownType,
        crate::UnknownType,
    >,
    initial: crate::UnknownType,
    object: Rc<Object>,
) -> crate::UnknownType {
    let keys: Vec<String> = object.entries().into_iter().map(|(key, _)| key).collect();
    let mut result = initial;
    for key in keys {
        // An eager bind may have deleted a later key during enumeration.
        if object.get(&key).is_none() {
            continue;
        }
        let object = object.clone();
        let callback = callback.clone();
        result = bind(result, purust_core::Func1::Shared(Rc::new(move |accumulator| {
            // Match g(k): read at continuation execution, not fold construction.
            // A property deleted after construction is JS undefined, not skipped.
            let value = object.get(&key).unwrap_or(crate::Value::Unit);
            callback(accumulator, key.clone(), value)
        })));
    }
    result
}

pub fn Foreign_Object_size(object: Rc<Object>) -> i64 {
    object.entries().len() as i64
}

pub fn Foreign_Object_keys(object: Rc<Object>) -> crate::UnknownType {
    let keys = object
        .entries()
        .into_iter()
        .map(|(key, _)| crate::Value::String(key))
        .collect();
    purust_core::mk_array(keys)
}

pub fn Foreign_Object_all(
    predicate: purust_core::Func2<String, crate::UnknownType, bool>,
    object: Rc<Object>,
) -> bool {
    // Snapshot the keys first: the predicate may update the object.
    for (key, _) in object.entries() {
        if let Some(value) = object.get(&key) {
            if !predicate(key, value) {
                return false;
            }
        }
    }
    true
}

pub fn Foreign_Object__fmapObject() -> crate::UnknownType {
    purust_object_function(move |object| {
        let object_for_map = object.clone();
        purust_object_function(move |callback| {
            let object = object_for_map.unwrap_class::<Rc<Object>>().clone();
            let callback = callback.unwrap_func1();
            // Read each value just before mapping it: the callback may update
            // later entries.
            let keys: Vec<String> = object.entries().into_iter().map(|(key, _)| key).collect();
            let mut mapped = Vec::with_capacity(keys.len());
            for key in keys {
                if let Some(value) = object.get(&key) {
                    mapped.push((key, callback(value)));
                }
            }
            crate::Value::Class(Rc::new(Rc::new(Object::from_entries(mapped))))
        })
    })
}

pub fn Foreign_Object__foldSCObject() -> crate::UnknownType {
    purust_object_function(move |object| {
        let object_for_z = object.clone();
        purust_object_function(move |initial| {
            let object_for_step = object_for_z.clone();
            let initial_for_step = initial.clone();
            purust_object_function(move |step| {
                let object_for_fold = object_for_step.clone();
                let initial_for_fold = initial_for_step.clone();
                let step_for_fold = step.clone();
                purust_object_function(move |from_maybe| {
                    let object = object_for_fold.unwrap_class::<Rc<Object>>().clone();
                    let step = step_for_fold.unwrap_func3();
                    let from_maybe = from_maybe.unwrap_func2();
                    let mut acc = initial_for_fold.clone();
                    for (key, value) in object.entries() {
                        let maybe = step(acc.clone(), crate::Value::String(key), value);
                        // `fromMaybe null` marks the early exit, like the JS FFI.
                        let next = from_maybe(acc.clone(), maybe);
                        if matches!(next.resolve(), crate::Value::Null) {
                            return acc;
                        }
                        acc = next;
                    }
                    acc
                })
            })
        })
    })
}

pub fn Foreign_Object__lookupST() -> crate::UnknownType {
    purust_object_function(move |nothing| {
        let nothing_for_just = nothing.clone();
        purust_object_function(move |just| {
            let nothing_for_key = nothing_for_just.clone();
            let just_for_key = just.clone();
            purust_object_function(move |key| {
                let nothing_for_object = nothing_for_key.clone();
                let just_for_object = just_for_key.clone();
                let key_for_object = key.unwrap_string();
                purust_object_function(move |object| {
                    let object_for_action = object
                        .unwrap_class::<Rc<Purs_Foreign_Object_ST::STObject>>()
                        .clone();
                    let nothing = nothing_for_object.clone();
                    let just = just_for_object.clone();
                    let key = key_for_object.clone();
                    purust_object_function(move |_| match object_for_action.get(&key) {
                        Some(value) => just.unwrap_func1()(value),
                        None => nothing.clone(),
                    })
                })
            })
        })
    })
}
