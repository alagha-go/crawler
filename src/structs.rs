#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum MovieType {
    #[default]
    Movie,
    Tvshow
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Movie {
    #[serde(serialize_with = "ObjectId::serialize")]
    pub id: ObjectId,
    pub name: String,
    pub movie_type: MovieType,
    pub code: String,
    pub page_url: String,
    pub image_url: String,
    pub description: String,
    pub genres: Vec<String>,
    pub trailer_url: String,
    pub released: String,
    pub duration: std::time::Duration,
    pub casts: Vec<String>,
    pub producers: Vec<String>,
    pub countries: Vec<String>,
}


#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Season {
    #[serde(serialize_with = "ObjectId::serialize")]
    pub id: ObjectId,
    name: String,
    code: String,
    episodes: Vec<Episode>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Episode {
    #[serde(serialize_with = "ObjectId::serialize")]
    pub id: ObjectId,
    number: i32,
    name: String,
    code: String,
    image_url: String,
    servers: Vec<Server>
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde(serialize_with = "ObjectId::serialize")]
    pub id: ObjectId,
    name: String,
    video_id: String
}
