use std::rc::Rc;
use Purs_Foreign_Object_ST::STObject;

pub fn Foreign_Object_ST_Unsafe_unsafeFreeze(object: Rc<STObject>) -> crate::UnknownType {
    // The frozen value is the same native map; the ST row type makes further
    // mutation unrepresentable, exactly like the JS implementation.
    crate::Value::Func1(purust_core::Func1::Shared(Rc::new(move |_| {
        crate::Value::Class(Rc::new(object.clone()))
    })))
}
