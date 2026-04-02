# Testing Guide — FoxTown Shopping Center

## Prerequisites

- Rust (stable) with `cargo`
- Dioxus CLI: `cargo install dioxus-cli`
- SQLite is embedded — no separate server needed

## Environment Variables

Create `.env` in the project root (already present in the repo):

```env
DATABASE_URL=sqlite:foxtown.db
JWT_SECRET=change-this-before-production
```

`DATABASE_URL` points to a local SQLite file created automatically on first run.
`JWT_SECRET` signs JWT session tokens — change it before deploying.

## Running Locally

```bash
dx serve
```

The dev server starts at <http://127.0.0.1:8080> with live reload and
auto-compiled Tailwind CSS.

## Default Credentials

A seed admin account is created on first startup:

| Username | Password | Role  |
|----------|----------|-------|
| `admin`  | `admin`  | Admin |

**Change the admin password before deploying to production.**

## Authentication Flow

1. Visit `/login` and sign in
2. The JWT is stored in `localStorage` under the key `auth_token`
3. On every page load, the `AuthProvider` component reads the token, calls
   the `me()` server function to validate it, and sets the global auth signal
4. The home page (`/`) requires authentication; unauthenticated users are
   redirected to `/login`
5. To log out: clear `localStorage` in the browser developer tools
   (a dedicated logout button is a planned feature)

## Daily Game Rules (enforced server-side)

- Each user may play once per day (`users.last_played_at`)
- One retry is allowed on the first attempt (`users.daily_attempts`)
- A maximum of 10 prizes can be awarded per day across all users
  (`daily_gifts` table, filtered by `awarded_at = CURRENT_DATE`)

## Build for Production

```bash
cargo build --release
```

Set `JWT_SECRET` and `DATABASE_URL` via environment variables (override `.env`).
