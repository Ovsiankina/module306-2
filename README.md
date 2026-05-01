# FoxTown Shopping Center Website

A modern, fullstack web application for the FoxTown Shopping Center built with Rust and Dioxus 0.7. The platform provides a comprehensive shop directory, interactive game system for prize redemption, visitor statistics, parking availability tracking, and an admin content management system.

## Features

### Core Features

- Shop Directory — Browse 160+ shops with advanced filtering by category, level, and name search
- Interactive Game System — Daily game with prize redemption vouchers (one per user per day with retry option)
- Parking Management — Real-time parking availability across 6 parking zones with EV charging station info
- Interactive Map — Visual floor plan editor to locate shops by level with drag-and-drop positioning
- Visitor Statistics — Track daily, monthly, and annual visitor counts
- User Authentication — Secure JWT-based authentication with role-based access control
- Admin CMS — Manage news, events, promotional banners, and shop information overlays
- Multi-Language Support — French, Italian, English, and German localization
- Responsive Design — Mobile-first design with Tailwind CSS
- Accessibility — WCAG-compliant components

### Technical Highlights

- Fullstack Dioxus — Client renders to WebAssembly, server runs in Rust
- Server-Side Rendering — Initial page load via SSR, subsequent interactions via LiveView
- Type-Safe APIs — Dioxus server functions provide type-safe, auto-serialized endpoints
- Embedded Data — 160 FoxTown shops and parking config baked in at compile-time
- SQLite Database — Lightweight, file-based database with automatic schema migrations

## Prerequisites

### Required

- Rust 1.70+ (https://rustup.rs/)
- Dioxus CLI 0.7.6+:
  ```bash
  cargo install dioxus-cli
  ```
- Node.js 16+ (for Tailwind CSS integration)

### Optional

- SQLite CLI (for manual database inspection)
- cargo-watch (for auto-recompilation during development)

## Installation

### 1. Clone the Repository

```bash
git clone <repository-url>
cd foxtown-shopping-center
```

### 2. Install Dependencies

```bash
cargo fetch
npm install  # If Tailwind is configured via Node.js
```

### 3. Configure Environment

Create a `.env` file in the project root:

```env
DATABASE_URL=sqlite:foxtown.db
JWT_SECRET=your-secret-key-here-change-in-production
```

**Important**: Change `JWT_SECRET` before deploying to production. Use a strong, random value.

### 4. Build the Project

```bash
cargo build
```

## Getting Started

### Development Server

Start the development server with hot reload enabled:

```bash
dx serve
```

The application will be available at `http://127.0.0.1:8080`.

- Live reload enabled for `.rs`, `.css`, and `.html` changes
- Tailwind CSS auto-compiled and watched
- WASM recompiled on client code changes
- Backend recompiled on server code changes

### Default Credentials

On first startup, a seed admin account is automatically created:

| Field | Value |
|-------|-------|
| Username | admin |
| Password | admin |
| Role | Admin |

**Important**: Change the admin password before deploying to production.

### Authentication Flow

1. Visit `http://127.0.0.1:8080/login`
2. Enter credentials and submit
3. JWT token is stored in browser localStorage under auth_token
4. On page reload, AuthProvider validates the token and restores the session
5. To logout: clear localStorage or use the logout function

## Testing

### Prerequisites for Testing

Same as development. No additional test database setup required.

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name -- --exact
```

### Manual Testing Checklist

1. Registration & Login
   - Register a new account at /register
   - Login with the new account
   - Verify JWT is stored in localStorage
   - Login again with admin / admin

2. Shop Directory
   - Navigate to /map
   - Filter by category (High Fashion, Casualwear, etc.)
   - Search by shop name
   - Sort by name or floor level
   - Click a shop card to view details

3. Game System
   - Play the daily game on the home page
   - Verify one game per day limit (subsequent attempts show "Already played today")
   - If first attempt loses, verify retry option appears
   - Verify max 10 prizes awarded per day across all users

4. Admin Features (requires Admin role)
   - Navigate to /admin
   - Create a news article
   - Create an event with date range
   - Create and activate a promotional banner
   - Edit shop information overlay
   - Verify changes persist across page reloads

5. Parking
   - Navigate to /parking
   - View availability for all 6 zones (P1, P2, P3, FoxPark, P5, P6)
   - View EV charging station details
   - (Admin) Update zone occupancy manually

6. Visitor Statistics (Admin only)
   - Navigate to /stats
   - View daily, monthly, and annual visitor counts
   - Verify data persists

7. Map Editor (Admin only)
   - Navigate to /admin/map
   - Drag shops to position them on the floor plan
   - Verify positions save to database

## API Documentation

Complete API documentation is provided in dedicated files:

- ADMIN_API.md — Authentication, news, events, banners, shop info
- PARKING_API.md — Parking system state, zone updates, vehicle entry/exit

### Example: Login

Request:
```bash
curl -X POST http://127.0.0.1:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin"}'
```

Response:
```json
"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwidXNlcm5hbWUiOiJhZG1pbiIsImVtYWlsIjoiYWRtaW5AZXhhbXBsZS5jb20iLCJyb2xlIjoiQWRtaW4ifQ.signature"
```

Store this token and pass it as token in subsequent API calls:

```bash
curl -X POST http://127.0.0.1:8080/api/create_news \
  -H "Content-Type: application/json" \
  -d '{
    "token":"<jwt-token>",
    "title":"New Sale",
    "body":"<p>Summer collection on sale now!</p>"
  }'
```

## Project Structure

```
foxtown-shopping-center/
├── src/
│   ├── main.rs                 # Router, providers, app entry
│   ├── stores.rs               # Store data types & server functions
│   ├── auth/                   # Authentication module (JWT, login, register)
│   ├── admin/                  # Admin CMS server functions
│   ├── db.rs                   # Database setup & schema
│   ├── api.rs                  # Game, stats, and other APIs
│   ├── services/
│   │   └── parking.rs          # Parking system logic
│   ├── components/             # UI components
│   │   ├── home.rs             # Private home page
│   │   ├── landing.rs          # Public landing page
│   │   ├── directory.rs        # Shop listing & filtering
│   │   ├── store_page.rs       # Shop detail page
│   │   ├── login.rs            # Login/register forms
│   │   ├── admin_content.rs    # Admin CMS interface
│   │   ├── parking.rs          # Parking display
│   │   ├── parking_admin.rs    # Admin parking editor
│   │   ├── map_editor.rs       # Interactive floor plan editor
│   │   ├── rewards_draw.rs     # Daily game UI
│   │   ├── visits_stats.rs     # Visitor stats dashboard
│   │   └── [other components]
│   ├── context/
│   │   ├── auth.rs             # Auth context provider
│   │   └── cart.rs             # Cart context provider
│   └── i18n/                   # Localization (FR, IT, EN, DE)
├── public/
│   ├── brands/                 # Store logo images
│   └── [other static assets]
├── migrations/
│   └── seeders/                # Embedded JSON data
│       ├── stores.json         # 160 FoxTown shops
│       └── parkings.json       # Parking configuration
├── Cargo.toml                  # Rust dependencies
├── tailwind.config.js          # Tailwind CSS config
├── .env                        # Environment variables
├── CLAUDE.md                   # Development guide
├── ADMIN_API.md                # Admin API reference
├── PARKING_API.md              # Parking API reference
└── README.md                   # This file
```

## Database

### SQLite

The application uses SQLite with automatic schema initialization. The database file is created at the path specified in DATABASE_URL:

```env
DATABASE_URL=sqlite:foxtown.db
```

### Schema Overview

| Table | Purpose |
|-------|---------|
| users | User accounts, roles, game state |
| news | News articles and announcements |
| events | Event listings with dates |
| banners | Promotional banners (can be activated/deactivated) |
| shop_info | CMS overlays extending static shop data |
| daily_gifts | Prize awards (tracked per day to enforce max 10/day) |
| visitor_stats | Daily, monthly, and annual visitor counts |

### Accessing the Database

Using SQLite CLI:

```bash
sqlite3 foxtown.db
sqlite> SELECT * FROM users;
sqlite> .schema
sqlite> .quit
```

## Build for Production

### Release Build

```bash
cargo build --release
```

Output: `target/release/ecommerce-site` (executable)

### Environment Variables

Set via environment (overrides .env):

```bash
export DATABASE_URL="sqlite:/data/foxtown.db"
export JWT_SECRET="your-production-secret"
./target/release/ecommerce-site
```

### Deployment

1. Build release executable
2. Copy executable to production server
3. Set environment variables
4. Ensure database directory exists and is writable
5. Run the executable

Recommended deployment options:
- Linux VPS with systemd service
- Docker (containerize the release build)
- Railway, Fly.io, or other Rust-friendly platforms

## Brand Logo Management

Store cards display logos from `public/brands/`:

- Location: `public/brands/<filename>.jpg`
- Default naming: Lowercase store name with spaces/special chars converted to underscores (e.g., "H&M" becomes `h_m.jpg`)
- Custom overrides: Some stores have explicit mappings in `src/components/home.rs` for accented names or special cases
- Missing logos: Stores without images show text placeholders

### Adding a New Store Logo

1. Add image file to `public/brands/` (preferred format: .jpg)
2. If store name does not auto-normalize to the filename, add an override in the `brand_image()` function
3. If image is unavailable, add the store to the "known missing" list to skip 404 errors

See the Brand Images section of CLAUDE.md for detailed instructions.

## Security Considerations

- JWT Secret: Change JWT_SECRET in production
- Password Storage: Passwords hashed with Argon2
- HTTPS: Deploy behind HTTPS (recommended for production)
- Session Cookies: Currently using localStorage. Consider switching to HTTP-only secure cookies for XSS mitigation
- HTML Sanitization: CMS fields accept raw HTML. Consider adding HTML sanitization with the ammonia crate
- SQL Injection: Protected by SQLx compile-time query checking
- CSRF Protection: Implement CSRF tokens if cookie-based sessions are added

## Features Status

| Feature | Status | Notes |
|---------|--------|-------|
| Shop Directory | Complete | With filtering and search |
| User Authentication | Complete | JWT-based, seed admin included |
| Game System | Complete | Daily limit, retry, max 10 prizes/day enforced |
| Admin CMS | Complete | News, events, banners, shop overlays |
| Parking Display | Complete | 6 zones, EV charging info, admin updates |
| Visitor Statistics | Complete | Daily/monthly/annual tracking |
| Interactive Map | In Progress | Editor complete, client display incomplete |
| Multi-language | Complete | FR, IT, EN, DE via i18n context |
| Responsive Design | Complete | Mobile-first with Tailwind CSS |
| Sign-up Flow | In Progress | Validation and UX refinements pending |
| OAuth Integration | Planned | Google and Apple sign-in |
| Session Persistence | Planned | Migrate from localStorage to secure cookies |

## Known Issues & TODOs

See CLAUDE.md for detailed technical TODOs.

Key items:
- Interactive map: Full client-side map rendering with search/filter
- Session storage: Switch to HTTP-only cookies
- HTML sanitization: Sanitize CMS content
- Banner images: File upload endpoint (S3/R2)
- Registration: Restrict to admin invites only

---
