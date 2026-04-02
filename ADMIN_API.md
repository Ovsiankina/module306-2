# Admin API Reference

All endpoints are Dioxus server functions exposed via `POST /api/<function_name>`.
Requests and responses are JSON-encoded.

Authentication uses a **JWT** returned by `login` or `register`.
Store it in `localStorage` (key `auth_token`) and pass it as `token` in every
protected call.

---

## Authentication (`src/auth/mod.rs`)

### Roles

| Role | Permissions |
|------|-------------|
| `Admin` | Full access — can create/update/delete everything and manage users |
| `Editor` | Can create and update content, but cannot delete or manage users |

### Endpoints

#### `POST /api/register`
Create a new account and receive a JWT (user is immediately logged in).

```json
{ "username": "alice", "email": "alice@example.com", "password": "secret" }
```
Returns: `"<jwt>"` (string). New accounts are created with the `Editor` role.

---

#### `POST /api/login`
Authenticate and receive a JWT.

```json
{ "username": "admin", "password": "admin" }
```
Returns: `"<jwt>"` (string).

Store the token in `localStorage` and pass it to all protected calls.

Development seed: `admin` / `admin` (Admin role).

---

#### `POST /api/logout`
No-op on the server (JWT is stateless). Client must clear `localStorage`.

```json
{ "token": "<jwt>" }
```
Returns: `null`.

---

#### `POST /api/me`
Decode a JWT and return the associated user, or `null` if invalid/expired.

```json
{ "token": "<jwt>" }
```
Returns: `{ "id": 1, "username": "alice", "email": "...", "role": "Editor" }` or `null`.

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

- Persist CMS content (news, events, banners, shop info) to the SQLite database
- Switch from `localStorage` JWT to an HTTP-only secure cookie (XSS protection)
- Restrict `register` to Admin callers only (currently open)
- Sanitize HTML body/description fields server-side (e.g. `ammonia` crate)
- Replace `image_url` in banners with a file-upload endpoint
