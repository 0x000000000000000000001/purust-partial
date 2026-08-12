pub fn Partial__crashWith(msg: crate::UnknownType) -> crate::UnknownType {
    panic!("Partial.crashWith: {}", msg.init_string.unwrap_or(""))
}

pub fn Partial_Unsafe__unsafePartial() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a { call: Some(std::rc::Rc::new(move |mut f: crate::UnknownType| -> crate::UnknownType {
        f.call.clone().unwrap()(crate::UnknownType::new(crate::Record_a { ..Default::default() }))
    })), ..Default::default() })
}
