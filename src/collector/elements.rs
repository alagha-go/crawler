pub fn get_servers(html: &String, servers: &mut Vec<Server>) -> Result<()> {
    let document = &Dom::new(html)?.documents[0];
    let documents = document.get_all_elements_by_name(&"a".to_owned());

    for document in documents {
        let mut server = Server::default();
        server.video_id = match document.child_attribute(0, &"data-id".to_owned()) {
            Some(value) => String::from(value),
            None => return Err(Error::new("Error: could not get data-id for server".to_owned()))?
        };
        let span = match document.get_element_by_name(&"span".to_owned(), 0) {
            Some(value) => value,
            None => return Err(Error::new("Error: could not get span for server".to_owned()))?
        };
        server.name = match span.text() {
            Some(value) => String::from(value),
            None => return Err(Error::new("Error: could not get server name".to_owned()))?
        };
        servers.push(server);
    }

    Ok(())
}

pub fn get_seasons(html: &String, seasons: &mut Vec<Season>) -> Result<()> {
    let document = &Dom::new(html)?.documents[0];
    let documents = document.get_all_elements_by_name(&"a".to_owned());

    for document in documents {
        let mut season = Season::default();
        season.code = match document.attribute(&"data-id".to_owned()) {
            Some(value) => String::from(value),
            None => return Err(Error::new("Error: could not get data id for seasons".to_owned()))?
        };
        let name_div = match document.get_element_by_name(&"a".to_owned(), 0) {
            Some(value) => value,
            None => return Err(Error::new("Error: could not get a's for seasons".to_owned()))?
        };
        season.name = match name_div.text() {
            Some(value) => String::from(value),
            None => return Err(Error::new("Error: could not get text for server's name_div".to_owned()))?
        };
        seasons.push(season);
    }

    Ok(())
}

pub fn get_episodes(html: &String, episodes: &mut Vec<Episode>) -> Result<()> {
    let document = &Dom::new(html)?.documents[0];
    let documents = document.get_all_elements_by_class(&"swiper-slide".to_owned());

    for document in documents {
        let mut episode = Episode::default();
        let image = match document.get_element_by_name(&"img".to_owned(), 0) {
            Some(value) => value,
            None => return Err(Error::new("Error: could not get img div for episodes".to_owned()))?
        };
        episode.image_url = match image.attribute(&"src".to_owned()) {
            Some(value) => String::from(value),
            None => return Err(Error::new("Error: could not get img's src for episodes".to_owned()))?
        };
        episode.code = match document.child_attribute(0, &"data-id".to_owned()) {
            Some(value) => String::from(value),
            None => return Err(Error::new("Error: could not get data-id for episodes".to_owned()))?
        };
        let episode_doc = match document.get_element_by_class(0, &"episode-number".to_owned()) {
            Some(value) => value,
            None => return Err(Error::new("Error: could not get episode number".to_owned()))?
        };
        let number_string = match episode_doc.text() {
            Some(value) => value,
            None => return Err(Error::new("Error: could not get episode_number_string".to_owned()))?
        };
        episode.number = number_string.replace(":", "").replace("Episode ", "").replace("\n", "").parse::<i32>()?;
        let film_name = match document.get_element_by_class(0, &"film-name".to_owned()) {
            Some(value) => value,
            None => return Err(Error::new("Error: could not get film-name for episodes".to_owned()))?
        };
        episode.name = match film_name.child_attribute(0, &"title".to_owned()) {
            Some(value) => String::from(value),
            None => return Err(Error::new("Error: could not get film-name title for episodes".to_owned()))?
        };
        episodes.push(episode);
    }
    Ok(())
}