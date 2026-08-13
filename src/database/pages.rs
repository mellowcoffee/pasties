use sqlx::FromRow;

use crate::model::pages::Page;
use crate::utility::datetime_now;
use crate::validation::page::{Css, Html, Slug};
use crate::State;

pub async fn get_page_by_slug(
    slug: &Slug,
    state: &State,
) -> Result<Option<Page>, sqlx::Error> {
    let page: Option<Page> = sqlx::query(
        "SELECT (id, slug, owner_id, html, css, views, created_at, updated_at)
        FROM pages
        WHERE slug = $1",
    )
    .bind(slug)
    .fetch_optional(&state.pool)
    .await?
    .map(|row| Page::from_row(&row))
    .transpose()?;
    Ok(page)
}

pub async fn insert_page(
    id: i64,
    slug: Slug,
    owner_id: i64,
    html: Html,
    css: Css,
    state: &State,
) -> Result<Page, sqlx::Error> {
    let page: Page = sqlx::query_as::<_, Page>(
        "
        INSERT INTO pages (id, slug, owner_id, html, css)
        VALUES ($1, $2, $3, $4, $5)
    ",
    )
    .bind(id)
    .bind(slug)
    .bind(owner_id)
    .bind(html)
    .bind(css)
    .fetch_one(&state.pool)
    .await?;
    Ok(page)
}

pub async fn update_page_by_slug(
    slug: Slug,
    new_slug: Slug,
    new_html: Html,
    new_css: Css,
    state: &State,
) -> Result<Page, sqlx::Error> {
    let page: Page = sqlx::query_as::<_, Page>(
        "
        UPDATE pages
        SET slug = $1, html = $2, css = $3, updated_at = $4
        WHERE slug = $5
        RETURNING (id, slug, owner_id, html, css, views, created_at, updated_at)
    ",
    )
    .bind(new_slug)
    .bind(new_html)
    .bind(new_css)
    .bind(datetime_now())
    .bind(slug)
    .fetch_one(&state.pool)
    .await?;
    Ok(page)
}
