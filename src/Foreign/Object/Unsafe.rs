use std::rc::Rc;

pub fn Foreign_Object_Unsafe_unsafeIndex(
    object: Rc<Purs_Foreign_Object::Object>,
    key: String,
) -> crate::UnknownType {
    // Like the JS implementation, a missing key reads as `undefined`.
    object.get(&key).unwrap_or(crate::Value::Unit)
}
