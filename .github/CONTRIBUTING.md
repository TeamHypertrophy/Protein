# Contributing to Protein 🧬

[![forthebadge](https://forthebadge.com/images/featured/featured-built-with-love.svg)](https://forthebadge.com)

Thank you for your interest in contributing to Protein! This document provides guidelines and information for contributing to our Rust-based fitness API.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Style Guidelines](#code-style-guidelines)
- [Architecture Overview](#architecture-overview)
- [Database Guidelines](#database-guidelines)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Issue Guidelines](#issue-guidelines)

## Getting Started

Before contributing, please:

1. Read our [Code of Conduct](.github/CODE_OF_CONDUCT.md)
2. Check existing [issues](https://github.com/TeamHypertrophy/Protein/issues) and [pull requests](https://github.com/TeamHypertrophy/Protein/pulls)
3. Review our [roadmap](README.md#roadmap-) for planned features

## Development Setup

### Prerequisites

Ensure you have all the required tools installed as listed in [README.md](README.md#prerequisites-):

## Code Style Guidelines

### Rust Formatting

We use `rustfmt` with nightly features. Configuration is in [`rustfmt.toml`](rustfmt.toml).

```bash
cargo +nightly fmt

./scripts/format.bat  # Windows
```

### Naming Conventions

- **Functions**: Use `snake_case` (e.g., `create_workout_plan`, `add_workout`)
- **Structs**: Use `PascalCase` (e.g., `WorkoutPlan`, `CustomExercise`)
- **Constants**: Use `SCREAMING_SNAKE_CASE` (e.g., `ASSETS_AVATARS_PATH`)
- **Modules**: Use `snake_case` (e.g., `workout`, `custom`, `trainer`)

### File Organization

Follow the established directory structure:

```
src/
├── api/          # API route handlers
├── auth/         # Authentication & authorization
├── cache/        # Redis caching logic
├── catchers/     # Error handlers
├── constants/    # Application constants
├── db/           # Database connection
├── errors/       # Error types
├── fairings/     # Rocket fairings
├── models/       # Database models
└── utils/        # Utility functions
```

## Architecture Overview

### Models Pattern

Our models follow a consistent pattern with associated functions:

```rust
impl ModelName {
    pub async fn find(/* params */) -> Result<ModelName, Error> { /* */ }
    pub async fn all(connection: &mut Conn) -> Result<Vec<ModelName>, Error> { /* */ }
    pub async fn user_all(user: Uuid, connection: &mut Conn) -> Result<Vec<ModelName>, Error> { /* */ }
    pub async fn create(data: NewModelName, connection: &mut Conn) -> Result<ModelName, Error> { /* */ }
    pub async fn update(/* params */, data: UpdateModelName, connection: &mut Conn) -> Result<ModelName, Error> { /* */ }
    pub async fn delete(/* params */, connection: &mut Conn) -> Result<usize, Error> { /* */ }
}
```

### API Route Structure

API routes are organized by functionality:

- `/v1/users` - User management
- `/v1/exercises` - Exercise CRUD operations
- `/v1/exercises/custom` - Custom exercise management
- `/v1/workouts` - Workout operations
- `/v1/plans` - Workout plan management
- `/v1/trainers` - Trainer functionality
- `/v1/profile` - User profile management
- `/v1/logs` - Various logging endpoints

### Admin Routes

Admin-only routes are defined in the [`Admin`](src/utils/admin.rs) struct. When adding new admin routes:

1. Add the route name to the `routes` vector in [`src/main.rs`](src/main.rs)
2. Implement the route handler with admin authentication
3. Update the admin route list documentation

## Database Guidelines

### Migrations

Use Diesel for database migrations:

```bash
# Create a new migration
diesel migration generate migration_name

# Apply migrations
diesel migration run

# Revert migrations
diesel migration revert
```

### Model Definitions

Follow the established pattern for models:

```rust
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Queryable,
    Selectable,
    Serialize,
    Deserialize,
    Identifiable,
    Associations,
)]
#[diesel(primary_key(id_field))]
#[diesel(table_name = table_name)]
#[diesel(belongs_to(RelatedModel))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ModelName {
    // Fields here
}
```

### Error Handling

Use the established error pattern with tracing:

```rust
.map_err(|error| {
    tracing::error!("[DB]: {:?}", error);
    Error::Database(error.to_string())
})
```

### Response Types

- Use `Json<T>` for successful responses
- Follow REST conventions for status codes

## Documentation

### Email Templates

Email templates are located in [`templates/`](templates/) and use the Askama templating engine:

- Follow the established HTML structure
- Include the Hypertrophy logo and branding
- Use consistent styling across templates
- Templates are defined in [`src/utils/templates.rs`](src/utils/templates.rs)

## Pull Request Process

1. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make Changes**
   - Follow the code style guidelines
   - Add tests for new functionality
   - Update documentation as needed

3. **Pre-commit Checks**
   ```bash
   # These run automatically with pre-commit hooks
   cargo +nightly fmt --check
   cargo clippy -- -D warnings
   ```

4. **Commit Changes**
   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

5. **Push and Create PR**
   ```bash
   git push origin feature/your-feature-name
   ```

## Issue Guidelines

### Bug Reports

Include:
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, etc.)
- Relevant logs or error messages

### Feature Requests

Include:
- Clear description of the feature
- Use case or problem it solves
- Proposed implementation (if any)
- Check if it aligns with our [roadmap](README.md#roadmap-)

### Labels

We use these labels to categorize issues:
- `bug` - Something isn't working
- `enhancement` - New feature or request
- `documentation` - Improvements to documentation
- `good first issue` - Good for newcomers
- `help wanted` - Extra attention is needed

## Questions?

- Check existing [issues](https://github.com/TeamHypertrophy/Protein/issues)
- Review the [README](README.md)
- Contact the maintainers through GitHub issues

---

Thank you for contributing to Protein! 🧬💚

*Made with ❤️ by the Hypertrophy Team*
