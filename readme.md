> [!WARNING]
> pasties is a work-in-progress project. Commits may be partial. Use at your
> own discretion. Nothing here is stable, and the schema, routes, and
> configuration are all subject to change without notice.

## 🦀 pasties

A small web service for publishing static pages ("pastes"). Registered users
may upload pairs of HTML and CSS files and host them at a specified slug.

### Features

- [ ] User accounts with a simple profile: username, bio, and an avatar rendered from an external link.
- [ ] One HTML file and one CSS file per page, addressed by a global slug.
- [ ] A live, server-rendered preview that matches the published result exactly.
- [ ] Page ownership with transfer, slug changes, and deletion.
- [ ] Optional invite-gated registration, with codes minted by administrators.
- [ ] An admin dashboard for managing users and pages.

### Building and running

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
 
### Configuration
 
All configuration lives in a single TOML file. See `config.example.toml` for
the full set of options.

### License

```
Copyright (C) 2026 Maxwell

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
