use std::sync::LazyLock;
use std::io::prelude::*;

use base64::{Engine as _, engine::general_purpose};

use flate2::read::GzDecoder;

use std::collections::HashMap;

pub static DEFAULT_EXTRA_STRING: LazyLock<String> = LazyLock::new(|| {
    let string = String::from("29_29_29_40_29_29_29_29_29_29_29_29_29_29_29_29");
    
    return string;
});

macro_rules! object_prop_bool {
    ($key:expr, $name:ident) => {
        pub fn $name(&self) -> bool {
            self.raw.get($key).map_or(false, |value| value == "1")
        }
    };
}

macro_rules! object_prop_int {
    ($key:expr, $name:ident) => {
        pub fn $name(&self) -> Option<i32> {
            self.raw.get($key).and_then(|s| s.parse().ok())
        }
    };
}

#[derive(Clone)]
pub struct ObjectData {
    raw: HashMap<String, String>
}

impl ObjectData {
    pub fn new(raw: HashMap<String, String>) -> Self {
        ObjectData { raw }
    }

    pub fn id(&self) -> i32 {
        self.raw.get("1").unwrap_or(&String::new()).parse().unwrap_or(0)
    }
    
    pub fn x(&self) -> f64 {
        self.raw.get("2").unwrap_or(&String::new()).parse().unwrap_or(0.0)
    }
    
    pub fn y(&self) -> f64 {
        self.raw.get("3").unwrap_or(&String::new()).parse().unwrap_or(0.0)
    }

    object_prop_bool!("13", checked);
    object_prop_int!("80", item_block_id);
}

pub enum PortalSpeed {
    Slow,
    Normal,
    Medium,
    Fast,
    VeryFast
}

impl Into<f64> for PortalSpeed {
    fn into(self) -> f64 {
        match self {
            PortalSpeed::Slow => 251.16,
            PortalSpeed::Normal => 311.58,
            PortalSpeed::Medium => 387.42,
            PortalSpeed::Fast => 478.0,
            PortalSpeed::VeryFast => 576.0
        }
    }
}

pub fn id_to_portal_speed(id: i32) -> Option<PortalSpeed> {
    match id {
        200 => Some(PortalSpeed::Slow),
        201 => Some(PortalSpeed::Normal),
        202 => Some(PortalSpeed::Medium),
        203 => Some(PortalSpeed::Fast),
        1334 => Some(PortalSpeed::VeryFast),
        _ => None,
    }
}

pub fn get_seconds_from_xpos(pos: f64, start_speed: PortalSpeed, portals: Vec<ObjectData>) -> f64 {
    let mut speed: f64;
    let mut last_obj_pos = 0.0;
    let mut last_segment = 0.0;
    let mut segments = 0.0;

    speed = start_speed.into();

    if portals.is_empty() {
        return pos / speed
    }

    for portal in portals {
        let mut s = portal.x() - last_obj_pos;

        if pos < s {
            s = s / speed;
            last_segment = s;
            segments += s;

            speed = id_to_portal_speed(portal.id()).expect("not a portal").into();

            last_obj_pos = portal.x()
        }
    }

    return ((pos - last_segment) / speed) + segments;
}

pub fn measure_length(objects: Vec<ObjectData>, ka4: i32) -> f64 {
    let start_speed = match ka4 {
        0 => PortalSpeed::Normal,
        1 => PortalSpeed::Slow,
        2 => PortalSpeed::Medium,
        3 => PortalSpeed::Fast,
        4 => PortalSpeed::VeryFast,
        _ => PortalSpeed::Normal
    };

    let max_x_pos = objects
        .iter()
        .fold(0.0, |max_x, obj| f64::max(max_x, obj.x()));

    let mut portals: Vec<ObjectData> = objects
        .into_iter()
        .filter(|obj| id_to_portal_speed(obj.id()).is_some() && obj.checked())
        .collect();

    portals.sort_by(|a, b| a.x().partial_cmp(&b.x()).unwrap());

    return get_seconds_from_xpos(max_x_pos, start_speed, portals)
}

pub fn secs_to_time(time: f64) -> i32 {
    match time {
        time if time < 10.0 => return 0,
        time if time < 30.0 => return 1,
        time if time < 60.0 => return 2,
        time if time < 120.0 => return 3,
        time if time >= 120.0 => return 4,
        _ => 0
    }
}

pub fn array_to_hash(arr: Vec<String>) -> HashMap<String, String> {
    return arr.chunks(2)
        .map(|chunk| (chunk[0].clone(), chunk[1].clone()))
        .collect()
}

pub fn parse(raw_level_data: &str) -> Vec<HashMap<String, String>> {
    raw_level_data
        .trim_end_matches(';')
        .split(';')
        .map(|v| {
            let values: Vec<String> = v.split(',').map(|s| s.to_string()).collect();
            array_to_hash(values)
        })
        .collect()
}

pub fn decode(level_data: String) -> Vec<HashMap<String, String>> {
    let decoded_bytes = general_purpose::URL_SAFE.decode(level_data).expect("couldnt decode b64");

    let mut decoder = GzDecoder::new(&decoded_bytes[..]);

    let mut uncompressed_data = String::new();
    decoder.read_to_string(&mut uncompressed_data).expect("err unzipping level");

    return parse(uncompressed_data.as_str())
}

pub fn to_objectdata(objects: Vec<HashMap<String, String>>) -> Vec<ObjectData> {
    return objects
        .into_iter()
        .filter(|v| v.contains_key("1"))
        .map(|v| ObjectData::new(v))
        .collect()
}