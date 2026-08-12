pub fn Partial_Unsafe_unsafePartial(f: crate::UnknownType) -> crate::UnknownType {
    let fn_val = f.clone().call.clone().unwrap();
    fn_val(crate::UnknownType::new(Record_a { ..Default::default() }))
}
