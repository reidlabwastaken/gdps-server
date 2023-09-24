use rocket_dyn_templates::{Template, context};

use rand::Rng;

#[get("/")]
pub fn index() -> Template {
    let silly_strings: Vec<&str> = vec![
        "the trianges consume",
        "geomtry das",
        "now with no ACE!",
        "the best gdps",
        "better than topala",
        "better than robtop",
        "slaughterhouse",
        "deepwoken verse 3",
        "skibidi toilet",
        "kagepro",
        "wowaka is peak music",
        "you have been warned: dyno jun",
        "listen to jin",
        "GIVEUP!GIVEUP!GIVEUP!GIVEUP!GIVEUP!GIVEUP!LOVE!LOVE!GIVEUP!GIVEUP!GIVEUP!GIVEUP!GIVEUP!GIVEUP!",
        "cross site scripting is a myth",
        "VITAL STATE: Deceased - abducted by Pikmin",
        "geometry dash for the 3ds",
        "trans rights",
        "how many maggots eat burger?",
        "who would win: the rust borrow checker or rotting flesh",
        "your system has run out of application memory",
        "unsafe { std::ptr::null_mut::<i32>().write(42) }",
        "-1",
        "[REDACTED]",
        "chrome jop jop?",
        "pikmin 4",
        "italian apk downloader",
        "Invalid password, can still smell him off the dashboard"
    ];

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..silly_strings.len());

    let silly_string = silly_strings[random_index];

    Template::render("index", context! { silly_string: silly_string })
}
