import axios from 'axios';
import type {
  Client,
  Interface,
  PeerStats,
  SessionResponse,
  ConfigResponse,
  LoginRequest,
  CreateClientRequest,
  UpdateClientRequest,
} from './types';

const http = axios.create({ withCredentials: true });

export const api = {
  session: {
    async login(data: LoginRequest): Promise<SessionResponse> {
      return (await http.post<SessionResponse>('/api/session', data)).data;
    },
    async logout(): Promise<void> {
      await http.delete('/api/session');
    },
    async check(): Promise<SessionResponse> {
      return (await http.get<SessionResponse>('/api/session')).data;
    },
  },

  clients: {
    async list(): Promise<Client[]> {
      return (await http.get<Client[]>('/api/client')).data;
    },
    async create(data: CreateClientRequest): Promise<Client> {
      return (await http.post<Client>('/api/client', data)).data;
    },
    async get(id: string): Promise<Client> {
      return (await http.get<Client>(`/api/client/${id}`)).data;
    },
    async update(id: string, data: UpdateClientRequest): Promise<Client> {
      return (await http.put<Client>(`/api/client/${id}`, data)).data;
    },
    async remove(id: string): Promise<void> {
      await http.delete(`/api/client/${id}`);
    },
    async enable(id: string): Promise<void> {
      await http.put(`/api/client/${id}/enable`);
    },
    async disable(id: string): Promise<void> {
      await http.put(`/api/client/${id}/disable`);
    },
    qrcodeUrl(id: string): string {
      return `/api/client/${id}/qrcode.svg`;
    },
    confUrl(id: string): string {
      return `/api/client/${id}/configuration`;
    },
  },

  interface: {
    async get(): Promise<Interface> {
      return (await http.get<Interface>('/api/interface')).data;
    },
    async update(data: Partial<Interface>): Promise<void> {
      await http.put('/api/interface', data);
    },
  },

  stats: {
    async get(): Promise<PeerStats[]> {
      return (await http.get<PeerStats[]>('/api/stats')).data;
    },
  },

  config: {
    async get(): Promise<ConfigResponse> {
      return (await http.get<ConfigResponse>('/api/config')).data;
    },
    async update(data: Partial<ConfigResponse>): Promise<void> {
      await http.put('/api/config', data);
    },
  },
};
