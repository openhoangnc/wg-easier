# API Reference

## Authentication

### POST /api/session
Login.

**Request:**
```json
{ "username": "admin", "password": "secret", "totp_code": "123456" }
```

**Response:**
```json
{ "authenticated": true, "username": "admin" }
```

### GET /api/session
Check current session.

### DELETE /api/session
Logout.

---

## Clients

All endpoints require authentication.

### GET /api/client
List all clients.

### POST /api/client
Create a new client.

**Request:** `{ "name": "my-phone" }`

### GET /api/client/:id
Get a single client.

### PUT /api/client/:id
Update a client.

**Request:** `{ "name": "new-name", "enabled": true }`

### DELETE /api/client/:id
Delete a client and remove from WireGuard kernel.

### PUT /api/client/:id/enable
Enable client peer.

### PUT /api/client/:id/disable
Disable client peer (removes from WireGuard kernel).

### GET /api/client/:id/qrcode.svg
SVG QR code containing the client `.conf`.

### GET /api/client/:id/configuration
Download WireGuard `.conf` file.

---

## Interface

### GET /api/interface
Get WireGuard interface info (public key, port, CIDR).

### PUT /api/interface
Update interface settings.

---

## Stats

### GET /api/stats
Get peer stats (rx/tx bytes, last handshake).

**Response:**
```json
[
  {
    "public_key": "base64...",
    "rx_bytes": 102400,
    "tx_bytes": 204800,
    "last_handshake_secs": 1709000000
  }
]
```

---

## Config

### GET /api/config
Get current config (host, port, DNS, allowed IPs).

### PUT /api/config
Update runtime config (DNS, allowed IPs).

---

## Metrics

### GET /metrics
Prometheus metrics endpoint (no auth required).
