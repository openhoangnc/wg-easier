CREATE TABLE IF NOT EXISTS users (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  username      TEXT    NOT NULL UNIQUE,
  password_hash TEXT    NOT NULL,
  totp_secret   TEXT
);

CREATE TABLE IF NOT EXISTS interfaces (
  id          TEXT PRIMARY KEY,
  name        TEXT NOT NULL DEFAULT 'wg0',
  private_key TEXT NOT NULL,
  public_key  TEXT NOT NULL,
  listen_port INTEGER NOT NULL DEFAULT 51820,
  ipv4_cidr   TEXT NOT NULL DEFAULT '10.8.0.0/24',
  ipv6_cidr   TEXT
);

CREATE TABLE IF NOT EXISTS clients (
  id             TEXT PRIMARY KEY,
  name           TEXT    NOT NULL,
  public_key     TEXT    NOT NULL UNIQUE,
  preshared_key  TEXT    NOT NULL,
  ipv4           TEXT    NOT NULL,
  ipv6           TEXT,
  enabled        INTEGER NOT NULL DEFAULT 1,
  created_at     TEXT    NOT NULL,
  expires_at     TEXT,
  download_url   TEXT,
  one_time_link  TEXT
);

CREATE TABLE IF NOT EXISTS config (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
