> [!WARNING]
> pasties is a work-in-progress project. Commits may be partial. Use at your
> own discretion. Nothing here is stable, and the schema, routes, and
> configuration are all subject to change without notice.

### 🦀 pasties

A small web service for publishing static pages ("pastes"). Registered users
may upload pairs of HTML and CSS files and host them at a specified slug.

> [!NOTE]
> An earlier project of mine was also called pasties. The two are mostly
> unrelated, the name and some code is reused, little else is.

#### Features

- [ ] User accounts with a simple profile: username, bio, and an avatar rendered from an external link.
- [ ] One HTML file and one CSS file per page, addressed by a global slug.
- [ ] A live, server-rendered preview that matches the published result exactly.
- [ ] Page ownership with transfer, slug changes, and deletion.
- [ ] Optional invite-gated registration, with codes minted by administrators.
- [ ] An admin dashboard for managing users and pages.

#### Building and running

```sh
cp config.example.toml config.toml
cargo run -- config.toml
```

You can run the Postgres from a Docker container:

```sh
docker run --name pasties-pg \                                                                             
  -e POSTGRES_PASSWORD=pass \
  -e POSTGRES_USER=user \
  -e POSTGRES_DB=pasties \
  -p 5432:5432 \
  -d postgres:17
```
 
#### Configuration
 
All configuration lives in a single TOML file. See `config.example.toml` for
the full set of options.
