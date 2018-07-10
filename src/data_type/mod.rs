#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateWrapper {
    pub trip: Trip,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Trip {
    pub data: Vec<DayTemplate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DayTemplate {
    pub id: u32,
    pub title: String,
    pub presentation: String,
    pub content: Vec<TemplateContext>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateContext {
    pub author: String,
    pub paragraph: String,
    pub img1: String,
    pub img2: String,
    pub img3: String,
}