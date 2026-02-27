export interface Client {
  id: string;
  name: string;
  public_key: string;
  preshared_key: string;
  ipv4: string;
  ipv6?: string;
  enabled: number;
  created_at: string;
  expires_at?: string;
  download_url?: string;
  one_time_link?: string;
}

export interface Interface {
  id: string;
  name: string;
  public_key: string;
  listen_port: number;
  ipv4_cidr: string;
  ipv6_cidr?: string;
}

export interface PeerStats {
  public_key: string;
  rx_bytes: number;
  tx_bytes: number;
  last_handshake_secs?: number;
}

export interface SessionResponse {
  authenticated: boolean;
  username?: string;
}

export interface ConfigResponse {
  wg_host: string;
  wg_port: number;
  wg_default_dns: string;
  wg_allowed_ips: string;
  wg_default_address: string;
}

export interface LoginRequest {
  username: string;
  password: string;
  totp_code?: string;
}

export interface CreateClientRequest {
  name: string;
}

export interface UpdateClientRequest {
  name?: string;
  enabled?: boolean;
  expires_at?: string;
}
