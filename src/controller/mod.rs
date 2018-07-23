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

    let index_page = reg.render("index", &generate_trip_content()).unwrap();
    rouille::Response::html(index_page)
}

pub fn serve_article(req: &Request, id : &i32) -> Response {
    let mut reg = Handlebars::new();

    reg.register_template_file("photo", &Path::new("./content/template/photo.html.hbs")).unwrap();
    reg.register_template_file("article", &Path::new("./content/template/article.html.hbs")).unwrap();

    let article_page = reg.render("article", &generate_article_content(id)).unwrap();
    rouille::Response::html(article_page )

}

fn generate_trip_content() -> Trip {
    let mut file = File::open("./content/data/text_content.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let mut trip: Trip = serde_json::from_str(&data).unwrap();

    for day in trip.data.iter_mut() {

        for cont in day.content.iter_mut() {
            let c1 = &cont.clone();
            cont.img1 = get_only_name_from_images(&c1.img1);

            cont.img2 = get_only_name_from_images(&c1.img2);

            cont.img3 = get_only_name_from_images(&c1.img3);
        }
    }

    trip
}

fn get_only_name_from_images(image_path: &str) -> String {
    let p1 = Path::new(image_path);
    if let Some(f1) = p1.file_name() {
        f1.to_os_string().into_string().unwrap()
    }else{
        String::new()
    }

}

fn generate_article_content(id: &i32) -> DayTemplate {
    let mut file = File::open("./content/data/text_content.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let trip: Trip = serde_json::from_str(&data).unwrap();

    let day = trip.data.iter().filter(|x| x.id == *id as u32).map(|x| x.clone()).collect::<Vec<DayTemplate>>();

    if let Some(day) = day.get(0){
        let mut day = day.clone();

        for cont in day.content.iter_mut() {
            let c1 = &cont.clone();
            cont.img1 = get_only_name_from_images(&c1.img1);

            cont.img2 = get_only_name_from_images(&c1.img2);

            cont.img3 = get_only_name_from_images(&c1.img3);
        }


        day
    } else {
        DayTemplate{
            title: "".to_string(),
            presentation:  "".to_string(),
            id:0,
            content: vec![]
        }
    }
}

