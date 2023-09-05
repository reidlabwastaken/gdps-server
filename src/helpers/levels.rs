use std::sync::LazyLock;

pub static DEFAULT_EXTRA_STRING: LazyLock<String> = LazyLock::new(|| {
    let string = String::from("29_29_29_40_29_29_29_29_29_29_29_29_29_29_29_29");

    return string;
});