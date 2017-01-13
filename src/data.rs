extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    data: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Image{
    src : String,
    jour: i32,
}
