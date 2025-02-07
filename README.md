# Protein - Rust Web API 💚

[![forthebadge](https://forthebadge.com/images/featured/featured-built-with-love.svg)](https://forthebadge.com)

## Information 📄

This is `Protein`, the Rust API that is used to Serve `Hypertrophy`, A Fully Featured Gym Mobile App.

## Tech Stack 🍱

![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white) ![Redis](https://img.shields.io/badge/redis-%23DD0031.svg?style=for-the-badge&logo=redis&logoColor=white) ![DigitalOcean](https://img.shields.io/badge/DigitalOcean-%230167ff.svg?style=for-the-badge&logo=digitalOcean&logoColor=white) ![VS Code Insiders](https://img.shields.io/badge/VS%20Code%20Insiders-35b393.svg?style=for-the-badge&logo=visual-studio-code&logoColor=white) ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white) ![Git](https://img.shields.io/badge/git-%23F05033.svg?style=for-the-badge&logo=git&logoColor=white) ![GitHub](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white) ![GitHub Actions](https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white)

## Prerequisites 🧑‍💻

- [Vistual Studio Code Insiders](https://code.visualstudio.com/docs/?dv=win&build=insiders)
- [HTTPie](https://httpie.io/desktop)
- [Redis](https://redis.io/docs/latest/operate/oss_and_stack/install/install-redis/install-redis-on-windows/)
- [PostgreSQL 17](https://www.postgresql.org/download/windows/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/products/docker-desktop/)
- [Pre-Commit](https://pre-commit.com/)

## Building 🏗️

### Docker 🐳

Building with docker is *so* simple!

1. Create an `docker.env` file and follow `.env.example` to full out the variables.

2. Run `docker compose`:

    ```sh
    $ docker compose up -d --build
    ```

### Local 👷

1. Install All Prerequisites

2. Clone The Repo

    ```sh
    $ git clone https://github.com/TeamHypertrophy/Protein.git
    ```

3. Create Local Database

    ```sh
    $ psql -U postgres
    postgres=# CREATE DATABASE your_local_db;
    postgres=# \c your_local_db
    ```

4. Rename `example.env` -> `.env` and Fill Out Variables

5. Install and Setup Diesel

   ```sh
    $ cargo install diesel_cli --no-default-features --features postgres
   ```

6. Run Diesel Setup

    ```sh
    $ diesel setup
    ```

7. Start Redis Server

    ```sh
    $ sudo service redis-server start
    ```

8. Run Protein!

    ```sh
    $ protein
    ```

9. Access The API At `http://127.0.0.1:8000`

## Roadmap 🚚

- [x] Docker Containerization
- [x] Code Modularization
- [ ] `/workouts`, `/exercises` Routes
- [ ] Automatic CI/CD

> Most Roadmap Features are written in my own private [`obsidian`](https://obsidian.md/) notes :p

## Folders 🗃️

`├──`[`.cargo`](https://github.com/TeamHypertrophy/Protein/tree/dev/.cargo) — Cargo Configuration<br>
`├──`[`.github`](https://github.com/TeamHypertrophy/Protein/tree/dev/.github) — GitHub configuration including CI/CD workflows<br>
`├──`[`.vscode`](https://github.com/TeamHypertrophy/Protein/tree/dev/.vscode) — VSCode Related Settings and Extension Recommendations<br>
`├──`[`migrations`](https://github.com/TeamHypertrophy/Protein/tree/dev/migrations) — Database Migrations<br>
`├──`[`scripts`](https://github.com/TeamHypertrophy/Protein/tree/dev/scripts) — Utility Scripts<br>
`├──`[`src`](https://github.com/TeamHypertrophy/Protein/tree/dev/src) — Source Code<br>
