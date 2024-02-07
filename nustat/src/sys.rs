use clap::{crate_name, crate_version};

pub fn get_app_title() -> String {
    format!("{} v{}", crate_name!(), crate_version!())
}
