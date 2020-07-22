#[macro_use]
extern crate rouille;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate handlebars;

mod controller;
mod data_type;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use handlebars::Handlebars;
use data_type::{TemplateContext, DayTemplate, TemplateWrapper, Trip};


const HOST : &'static str = "0.0.0.0:8000";
// const HOST : &'static str = "127.0.0.1:8000";

fn main() {
    println!("Server is running on {}", HOST);
    rouille::start_server(HOST, move |req| {
        //Serve static file
        {
            let response = rouille::match_assets(req, ".");
            if response.is_success() {
                return response;
            }
        }
        router!(req,
            (GET)(/) => {
                controller::serve_index(&req)
                },
                _ => rouille::Response::empty_404()
            )
    },
    );
}
