pub(crate) fn option_false(v: &Option<bool>) -> bool {
    matches!(v, Some(false) | None)
}
