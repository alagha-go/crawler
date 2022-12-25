pub fn collect_movies(html: &String) -> Result<[Movie; MOVIES_PER_PAGE]> {
    let mut document = &Dom::new(html)?.documents[0];
    let time = std::time::Instant::now();
    let documents = match document.get_element_by_class(0, &String::from("film_list-wrap")) {
        Some(value) => value,
        _ => return Err(Box::new(Error::new(String::from("crate::collector::collect.rs::collect_movies: could not get \"film_list-wrap\""))))
    }.get_all_elements_by_class(&String::from("flw-item"));

    if documents.len() != MOVIES_PER_PAGE {
        return Err(Box::new(Error::new(String::from("crate::collector::collect.rs::collect_movies: unexpected number of \"flw-item\" documents"))))
    }

    let mut movies = [(); MOVIES_PER_PAGE].map(|_|{Movie::default()});

    for index in 0..MOVIES_PER_PAGE {
        document = documents[index];
        let film_detail = match document.get_element_by_class(0, &"film-detail".to_owned()) {
            Some(value) => value,
            None => return Err(Box::new(Error::new(String::from("crate::collector::collect.rs::collect_movies: could not get \"film-detail\""))))
        };
        let film_name = match film_detail.get_element_by_class(0, &"film-name".to_owned()) {
            Some(value) => value,
            None => return Err(Box::new(Error::new(String::from("crate::collector::collect.rs::collect_movies: could not get \"film-name\""))))
        };
        let poster = match document.get_element_by_class(0, &"film-poster".to_owned()) {
            Some(value) => value,
            None => return Err(Box::new(Error::new(String::from("crate::collector::collect.rs::collect_movies: could not get \"film-poster\""))))
        };
        movies[index].page_url = DOMAIN.to_owned() + match &film_name.child_attribute(0, &"href".to_owned()) {
            Some(value) => value,
            None => return Err(Box::new(Error::new(String::from("crate::collector::collect.rs::collect_movies: could not get \"page_url\""))))
        };
        let split: Vec<&str> = movies[index].page_url.split("-").collect();
        movies[index].code = split[split.len()-1].to_owned();
        movies[index].name = html_escape::decode_html_entities(match &film_name.child_attribute(0, &"title".to_owned()) {
            Some(value) => value,
            None => return Err(Box::new(Error::new(String::from("crate::collector::collect.rs::collect_movies: could not get \"movie_title\""))))
        }).to_string();
        movies[index].image_url = match poster.child_attribute(0, &"data-src".to_owned()) {
            Some(value) => String::from(value),
            None => return Err(Box::new(Error::new(String::from("crate::collector::collect.rs::collect_movies: could not get \"image_url\""))))
        };
    }

    println!("{:?}", time.elapsed());

    Ok(movies)
}


pub fn pages(html: &str, movie_type: MovieType) -> Result<usize> {
    let mut document = &Dom::new(html)?.documents[0];
    document = match document.get_element_by_attribute(0, "title", &String::from("Last")) {
        Some(value) => value,
        None => return Err(Box::new(Error::new(String::from("crate::collector::collect.rs::pages: could not get document by title"))))
    };
    let link = match document.attribute("href") {
        Some(value) => value,
        _ => return Err(Box::new(Error::new(String::from("crate::collector::collect.rs::pages: could not get document by attribute \"href\""))))
    };
    let link = match movie_type {
        MovieType::Movie => link.replace("/movie?page=", ""),
        _ => link.replace("/tv-show?page=", "")
    };
    Ok(link.parse::<usize>()?)
}