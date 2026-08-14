use maud::{html, Markup, DOCTYPE};

use crate::error::AppError;
use crate::model::pages::Page;
use crate::State;

pub fn layout(head: &Markup, body: &Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                (head)
            }
            body {
                (body)
            }
        }
    }
}

pub fn render_page(page: &Page) -> Markup {
    let head = html! {
        title { (page.slug) }
        style { (page.css) }
    };
    let body = html! {
        main { (page.html) }
    };
    layout(&head, &body)
}

pub fn render_error(err: &AppError) -> Markup {
    let head = html! {
        title { "Error" }
    };
    let body = html! {
        main { (format!("{}", err)) }
    };
    layout(&head, &body)
}

pub fn register(state: &State) -> Markup {
    let require_invite = state.config.registration.require_invite;
    let password_min_len = state.config.limits.password_min_len;
    let password_max_len = state.config.limits.password_max_len;
    let head = html! {
        title { "Register" }
    };
    let body = html! {
        h1 { "Register" }
        form method="post" action="/register" {
            label { "Username"
                input type="text" name="username" required
                      minlength="3" maxlength="32" pattern="[a-z0-9_-]";
            }
            label { "Password"
                input type="password" name="password" required
                    minlength=(password_min_len) maxlength=(password_max_len);
            }
            label { "Confirm password"
                input type="password" name="password_confirm" required
                    minlength=(password_min_len) maxlength=(password_max_len);
            }
            @if require_invite {
                label { "Invite code"
                    input type="text" name="invite_code" required;
                }
            }
            button type="submit" { "Create account" }
        }
        p { "Have an account? " a href="/login" { "Log in" } }
    };
    layout(&head, &body)
}

pub fn login() -> Markup {
    let head = html! {
        title { "Log in" }
    };
    let body = html! {
        h1 { "Log in" }
        form method="post" action="/login" {
            label { "Username"
                input type="text" name="username" required;
            }
            label { "Password"
                input type="password" name="password" required;
            }
            button type="submit" { "Log in" }
        }
        p { "No account?" a href="/register" { "Register" } }
    };
    layout(&head, &body)
}
