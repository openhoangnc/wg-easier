# Architecture

## Overview

```
                        ┌──────────────────────────────────────────────┐
                        │              Docker Container                 │
                        │             (FROM scratch)                    │
                        │                                               │
  Browser / WG Client   │   ┌──────────────────────────────────────┐   │
  ──────────────────►   │   │        wg-easy-rs binary              │   │
  :51821/tcp (HTTP)      │   │                                       │   │
                        │   │  ┌──────────────┐  ┌───────────────┐  │   │
  WireGuard Peers        │   │  │  Axum HTTP   │  │  React SPA    │  │   │
  ──────────────────►   │   │  │  API Server  │  │  (static)     │  │   │
  :51820/udp (WireGuard) │   │  └──────┬───────┘  └───────────────┘  │   │
                        │   │         │                               │   │
                        │   │  ┌──────▼───────────────────────────┐  │   │
                        │   │  │  Business Logic                   │  │   │
                        │   │  │  ┌──────────┐  ┌──────────────┐  │  │   │
                        │   │  │  │  SQLite  │  │  WireGuard   │  │  │   │
                        │   │  │  │  (sqlx)  │  │  (rtnetlink) │  │  │   │
                        │   │  │  └──────────┘  └──────────────┘  │  │   │
                        │   │  │  ┌──────────────────────────────┐  │  │   │
                        │   │  │  │  nftables NAT (rustables)   │  │  │   │
                        │   │  │  └──────────────────────────────┘  │  │   │
                        │   │  └──────────────────────────────────┘  │   │
                        │   └──────────────────────────────────────┘   │
                        └──────────────────────────────────────────────┘
```

## Components

### Backend (Rust)

- **`axum`** — async HTTP framework, serves both REST API and React SPA
- **`sqlx`** — async SQLite driver with compile-time query checking
- **`rtnetlink`** — Linux netlink sockets for WireGuard interface management
- **`wireguard-control`** — WireGuard kernel interface (peer management, stats)
- **`rustables`** — nftables bindings for NAT MASQUERADE
- **`tower-sessions`** — cookie-based session management
- **`bcrypt`** — password hashing / verification
- **`totp-rs`** — TOTP (2FA) support
- **`tera`** — template engine for `.conf` generation

### Frontend (React)

- **React 18** + **TypeScript**
- **Vite** — fast build tool
- **Tailwind CSS v4** — utility-first styling
- **TanStack Query** — server state management
- **React Router** — SPA routing
- **i18next** — internationalization (English, German)

### Docker Image

- Multi-stage build: Rust (musl) + Node.js → `FROM scratch`
- Final image: single binary + static React SPA
- Size: ~10–15 MB

## Data Flow

1. Browser loads React SPA from `/` (served by Axum's `ServeDir`)
2. SPA calls `/api/*` endpoints (same origin, no CORS issues in prod)
3. Session cookie maintained via `tower-sessions`
4. Client CRUD → DB write + WireGuard kernel update (atomic)
5. WireGuard stats polled every 10s via `/api/stats`
