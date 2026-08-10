pub fn Partial_Unsafe__unsafePartial(f: crate::UnknownType) -> crate::UnknownType {
    let fn_val = f.clone().into_rc_fn();
    fn_val(crate::UnknownType::new(0))
}
