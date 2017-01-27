#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub data: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub id: i32,
    pub src: String,
    pub jour: i32,
    pub text: String,
    pub coord: [f32; 2],
}
