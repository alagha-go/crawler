#[derive(Debug, Default, Clone, Copy)]
pub enum MovieType {
    #[default]
    Movie,
    Tvshow
}

#[derive(Debug, Default, Clone)]
pub struct Movie {
    pub name: String,
    pub movie_type: MovieType,
    pub code: String,
    pub page_url: String,
    pub image_url: String,
}