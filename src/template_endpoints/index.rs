use rocket_dyn_templates::{Template, context};

use rand::Rng;

#[get("/")]
pub fn index() -> Template {
    let silly_strings: Vec<&str> = vec![
        "the trianges consume",
        "geomtry das"
    ];

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..silly_strings.len());

    let silly_string = silly_strings[random_index];

    Template::render("index", context! { silly_string: silly_string })
}
