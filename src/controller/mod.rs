use rouille;
use rouille::{Request, Response};
use handlebars::Handlebars;
use handlebars;
use data_type::{TemplateWrapper, DayTemplate, TemplateContext, Trip};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde_json::Value;

pub fn serve_static_files(req: &Request) -> Response {
    let response = rouille::match_assets(req, ".");
    if response.is_success() {
        return response;
    } else {
        return Response::text("Erreur 404, fichier non trouvé").with_status_code(404);
    }
}

pub fn serve_index(req: &Request) -> Response {
    let mut reg = Handlebars::new();

    //    let mut source_file = File::open(&"./content/template/index.html.hbs").unwrap();
    reg.register_template_file("index", &Path::new("./content/template/index.html.hbs")).unwrap();
    reg.register_template_file("photo", &Path::new("./content/template/photo.html.hbs")).unwrap();
    reg.register_template_file("text", &Path::new("./content/template/text.html.hbs")).unwrap();

    let index_page = reg.render("index", &generate_content()).unwrap();
    rouille::Response::html(index_page)
}

pub fn serve_article(req: &Request) -> Response {
    rouille::Response::text("ok").with_status_code(200)
}

fn generate_content() -> Trip {
    let mut file = File::open("./content/data/text_content.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let trip: Trip = serde_json::from_str(&data).unwrap();

    trip
}