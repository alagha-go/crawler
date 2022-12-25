pub fn get_movie_content(html: &String, movie: &mut Movie) -> Result<()> {
    let document = &Dom::new(html)?.documents[0];
    let description_div = match document.get_element_by_class(0, &"description".to_owned()) {
        Some(value) => value,
        None => return Err(Error::new("Error: could not get description_div".to_owned()))?
    };
    movie.description = match description_div.text() {
        Some(value) => html_escape::decode_html_entities(&value).to_string(),
        _ => String::from("")
    };
    let trailer = match document.get_element_by_id(0, &"iframe-trailer".to_owned()) {
        Some(value) => value,
        None => return Err(Error::new("Error: could not get iframe-trailer".to_owned()))?
    };
    movie.trailer_url = match trailer.attribute(&"data-src".to_owned()) {
        Some(value) => value.clone(),
        None => return Err(Error::new("Error: could not get data-src".to_owned()))?
    };
    
    let element = match document.get_element_by_class(0, &"elements".to_owned()) {
        Some(value) => value,
        None => return Err(Error::new("Error: could not get elements".to_owned()))?
    };
    let elements = match element.get_element_by_class(0, &"row".to_owned()) {
        Some(value) => value,
        None => return Err(Error::new("Error: could not get row elements".to_owned()))?
    }.get_all_elements_by_class(&"row-line".to_owned());

    for element in &elements {
        let strong = match element.get_element_by_name(&"strong".to_owned(), 0) {
            Some(value) => value.clone(),
            None => return Err(Error::new("Error: could not get strong".to_owned()))?
        };
        let name = match strong.text() {
            Some(value) => value.clone(),
            None => return Err(Error::new("Error: could not get text for strong".to_owned()))?
        };

        if name == "Released:".to_owned() {
            movie.released = match element.text() {
                Some(value) => value.clone(),
                None => return Err(Error::new("Error: could not get released".to_owned()))?
            };
        } else if name == "Genre:".to_owned() {
            let values = element.get_all_elements_by_name(&"a".to_owned());
            for element in &values {
                movie.genres.push(html_escape::decode_html_entities(match &element.attribute(&"title".to_owned()) {
                    Some(value) => value,
                    None => return Err(Error::new("Error: could not get genres".to_owned()))?
                }).to_string());
            }
        } else if name == "Casts:".to_owned() {
            let values = element.get_all_elements_by_name(&"a".to_owned());
            for element in &values {
                movie.casts.push(html_escape::decode_html_entities(match &element.attribute(&"title".to_owned()) {
                    Some(value) => value,
                    None => return Err(Error::new("Error: could not get casts".to_owned()))?
                }).to_string());
            }
        } else if name == "Duration:".to_owned() {
            let text = match element.text() {
                Some(value) => value,
                None => return Err(Error::new("Error: could not get duration".to_owned()))?
            };
            let string = text.split("\n").collect::<Vec<&str>>()[0];
            let int: u64 = match string.parse() {
                Ok(value) => value,
                _ => 0
            };
            movie.duration = std::time::Duration::from_secs(int * 60);
        } else if name == "Country:".to_owned() {
            let values = element.get_all_elements_by_name(&"a".to_owned());
            for element in &values {
                movie.countries.push(html_escape::decode_html_entities(match &element.attribute(&"title".to_owned()) {
                    Some(value) => value,
                    None => return Err(Error::new("Error: could not get country".to_owned()))?
                }).to_string());
            }
        } else if name == "Production:".to_owned() {
            let values = element.get_all_elements_by_name(&"a".to_owned());
            for element in &values {
                movie.producers.push(html_escape::decode_html_entities(match &element.attribute(&"title".to_owned()) {
                    Some(value) => value,
                    None => return Err(Error::new("Error: could not get producers".to_owned()))?
                }).to_string());
            }
        }
    }

    Ok(())
}