# Admin API Reference

All endpoints are Dioxus server functions exposed via `POST /api/<function_name>`.
Requests and responses are JSON-encoded.

Authentication uses a **session token** returned by `login`. Pass it as `token` in every protected call.

---

## Authentication (`src/auth/mod.rs`)

### Roles

| Role | Permissions |
|------|-------------|
| `Admin` | Full access — can create/update/delete everything and manage users |
| `Editor` | Can create and update content, but cannot delete or manage users |

### Endpoints

#### `POST /api/register`
Create a new user account.
> TODO: restrict to Admin callers before going to production.

```json
{ "username": "alice", "password": "secret", "role": "Editor" }
```
Returns: `null` on success.

---

#### `POST /api/login`
Authenticate and receive a session token.

```json
{ "username": "admin", "password": "admin" }
```
Returns: `"<session-token>"` (string).

Store the token client-side and pass it to all protected calls.
> TODO: switch to HTTP-only secure cookies to prevent XSS token theft.

Development seed: `admin` / `admin` (Admin role).

---

#### `POST /api/logout`
Invalidate a session token.

```json
{ "token": "<session-token>" }
```
Returns: `null`.

---

#### `POST /api/whoami`
Get the user associated with a token, or `null` if the token is invalid.

```json
{ "token": "<session-token>" }
```
Returns: `{ "username": "alice", "role": "Editor" }` or `null`.

---

## News & Announcements (`src/admin/mod.rs`)

#### `POST /api/list_news` — public
Returns: `[NewsItem]`

```json
[{
  "id": 1,
  "title": "Summer Sale",
  "body": "<p>HTML content from WYSIWYG editor</p>",
  "author": "alice",
  "created_at": "2026-03-19T10:00:00Z",
  "updated_at": "2026-03-19T10:00:00Z"
}]
```

#### `POST /api/create_news` — Editor+
```json
{ "token": "...", "title": "Title", "body": "<p>HTML</p>" }
```
Returns: `NewsItem`.

#### `POST /api/update_news` — Editor+
```json
{ "token": "...", "id": 1, "title": "New title", "body": "<p>Updated HTML</p>" }
```
Returns: updated `NewsItem`.

#### `POST /api/delete_news` — Admin only
```json
{ "token": "...", "id": 1 }
```
Returns: `null`.

---

## Events (`src/admin/mod.rs`)

#### `POST /api/list_events` — public
Returns: `[Event]`

#### `POST /api/create_event` — Editor+
```json
{
  "token": "...",
  "title": "Summer Pop-up",
  "description": "<p>HTML</p>",
  "date": "2026-06-01",
  "end_date": "2026-06-03",
  "location": "Level 1 — Main Hall"
}
```
`end_date` is optional. Returns: `Event`.

#### `POST /api/update_event` — Editor+
Same fields as `create_event` plus `"id": <number>`. Returns: updated `Event`.

#### `POST /api/delete_event` — Admin only
```json
{ "token": "...", "id": 1 }
```
Returns: `null`.

---

## Banners (`src/admin/mod.rs`)

Banners are **inactive by default** when created. Activate them explicitly.

#### `POST /api/list_banners` — public
Returns only **active** banners sorted by `display_order`.

#### `POST /api/list_all_banners` — Editor+
```json
{ "token": "..." }
```
Returns all banners (including inactive), sorted by `display_order`.

#### `POST /api/create_banner` — Editor+
```json
{
  "token": "...",
  "title": "Spring Collection",
  "image_url": "https://example.com/banner.jpg",
  "link_url": "/map",
  "display_order": 1
}
```
`link_url` is optional. Returns: `Banner`.
> TODO: replace `image_url` with a file-upload endpoint (S3/Cloudflare R2).

#### `POST /api/set_banner_active` — Editor+
```json
{ "token": "...", "id": 1, "active": true }
```
Returns: `null`.

#### `POST /api/delete_banner` — Admin only
```json
{ "token": "...", "id": 1 }
```
Returns: `null`.

---

## Shop Info overlay (`src/admin/mod.rs`)

Extends the static `stores.json` data with CMS-managed fields, keyed by store **slug**
(e.g. `tommy-hilfiger`, `h-m`). See `src/stores.rs::slugify` for slug generation.

#### `POST /api/get_shop_info` — public
```json
{ "slug": "tommy-hilfiger" }
```
Returns: `ShopInfo` or `null` if not yet set.

```json
{
  "slug": "tommy-hilfiger",
  "description": "<p>HTML from WYSIWYG</p>",
  "opening_hours": "Mon–Sat 10:00–19:00, Sun 11:00–18:00",
  "special_notice": null,
  "updated_at": "2026-03-19T10:00:00Z",
  "updated_by": "alice"
}
```

#### `POST /api/upsert_shop_info` — Editor+
Creates or fully replaces the overlay for a store.
```json
{
  "token": "...",
  "slug": "tommy-hilfiger",
  "description": "<p>HTML from WYSIWYG</p>",
  "opening_hours": "Mon–Sat 10:00–19:00",
  "special_notice": "Closed 25 December"
}
```
`special_notice` is optional (`null` to clear it). Returns: `ShopInfo`.

---

## Known TODOs

- Persist all data to a real database (PostgreSQL or SQLite via SQLx)
- Replace SHA-256 password hashing with argon2 or bcrypt with per-user salts
- Replace session token in response body with HTTP-only secure cookie
- Restrict `register` to Admin callers only
- Sanitize HTML body/description fields server-side (e.g. `ammonia` crate)
- Replace `image_url` in banners with a file-upload endpoint
- Persist sessions to Redis/DB so they survive server restarts
