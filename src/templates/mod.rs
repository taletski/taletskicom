use askama::Template;

#[derive(Default, Template)]
#[template(path = "homepage.html")]
pub struct HomepageTemplate {}
