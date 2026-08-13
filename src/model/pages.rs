use chrono::Utc;
use sqlx::prelude::FromRow;

use crate::database::pages::{get_page_by_slug, insert_page, update_page_by_slug};
use crate::database::users::get_user_by_id;
use crate::error::{AppError, PageError, UserError};
use crate::validation::page::{Css, Html, Slug};
use crate::State;

#[derive(FromRow)]
pub struct Page {
    id:         i64,
    slug:       Slug,
    owner_id:   i64,
    html:       Html,
    css:        Css,
    views:      i64,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

pub struct CreatePage {
    slug:     String,
    owner_id: i64,
    html:     String,
    css:      String,
}

pub struct UpdatePage {
    slug:     String,
    new_slug: String,
    new_html: String,
    new_css:  String,
}

impl Page {
    pub async fn create_page(page: CreatePage, state: State) -> Result<Self, AppError> {
        let slug = Slug::parse(page.slug)?;
        let html = Html::parse(page.html, &state.config.limits)?;
        let css = Css::parse(page.css, &state.config.limits)?;

        if let Ok(None) = get_user_by_id(page.owner_id, &state).await {
            Err(UserError::InvalidId)?;
        }

        let id = state.snowflake.generate().to_i64();

        if get_page_by_slug(&slug, &state).await?.is_some() {
            Err(PageError::SlugTaken)?;
        }

        let page = insert_page(id, slug, page.owner_id, html, css, &state).await?;
        Ok(page)
    }

    pub async fn update_page(update_page: UpdatePage, state: State) -> Result<Self, AppError> {
        let slug = Slug::parse(update_page.slug)?;
        let _page = get_page_by_slug(&slug, &state)
            .await?
            .ok_or(PageError::NotFound)?;

        let new_slug = Slug::parse(update_page.new_slug)?;
        if get_page_by_slug(&new_slug, &state).await?.is_some() {
            Err(PageError::SlugTaken)?;
        }

        let new_html = Html::parse(update_page.new_html, &state.config.limits)?;
        let new_css = Css::parse(update_page.new_css, &state.config.limits)?;

        let new_page = update_page_by_slug(slug, new_slug, new_html, new_css, &state).await?;
        Ok(new_page)
    }
}
