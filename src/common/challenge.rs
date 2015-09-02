
pub struct Info {
    pub no:         u32,
    pub title:      &'static str,
    pub help:       &'static str,
    pub execute_fn: fn() -> i32
}

