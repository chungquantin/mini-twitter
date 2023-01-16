/// String to Static Str
pub fn sss(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
