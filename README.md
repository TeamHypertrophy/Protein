# Protein - Rust Web API 💚

## Information 📄

This is `Protein`, the Rust API that is used to Serve `Hypertrophy`, A Fully Featured Gym Mobile App.

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

Building with docker is *so* simple! Just Run:

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

## Contributing 👥

> Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

Before Creating Pull Requests, Ensure That Code Is Run Through:

- `cargo fmt`
- `cargo clippy`
- `pre-commit`

1. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
2. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
3. Push to the Branch (`git push origin feature/AmazingFeature`)
4. Open a Pull Request

## Routes 🗃️

`health`

- `/redis`
- `/postgres`

`system`

- `/`
- `/rust`
- `/package`
- `/git`

`users`

- `v1/<user_id>`
- `v1/all`
- `v1/create`
- `v1/delete/<user_id>`
- `v1/update/<user_id>`
- `v1/me`
