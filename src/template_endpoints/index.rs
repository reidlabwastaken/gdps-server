use rocket_dyn_templates::{Template, context};

use rand::Rng;

#[get("/")]
pub fn index() -> Template {
    let silly_strings: Vec<&str> = vec![
        "the trianges consume",
        "geomtry das",
        "now with no RCE!",
        "the best gdps",
        "better than topala",
        "better than robtop",
        "slaughterhouse",
        "deepwoken verse 3",
        "skibidi toilet",
        "kagepro",
        "wowaka is peak music",
        "you have been warned: dyno jun",
        "listen to jin"
    ];

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..silly_strings.len());

    let silly_string = silly_strings[random_index];

    Template::render("index", context! { silly_string: silly_string })
}
