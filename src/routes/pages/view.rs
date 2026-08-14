use maud::{html, Markup, DOCTYPE};

use crate::error::AppError;
use crate::model::pages::Page;

pub struct Layout {
    pub head: Option<Markup>,
    pub body: Option<Markup>,
}

pub fn render_page(page: &Page) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (page.slug) }
                style { (page.css) }
            }
            body {
                main { (page.html) }
            }
        }
    }
}

pub fn render_error(err: &AppError) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Error" }
            }
            body {
                main { (format!("{}", err)) }
            }
        }
    }
}
