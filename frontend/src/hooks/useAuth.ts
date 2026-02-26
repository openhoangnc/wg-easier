import { useState, useEffect } from 'react';
import { api } from '../api/client';

export function useAuth() {
  const [authenticated, setAuthenticated] = useState<boolean | null>(null);
  const [username, setUsername] = useState<string | undefined>();

  useEffect(() => {
    api.session.check().then((r) => {
      setAuthenticated(r.authenticated);
      setUsername(r.username);
    });
  }, []);

  const login = async (username: string, password: string, totp_code?: string) => {
    const r = await api.session.login({ username, password, totp_code });
    setAuthenticated(r.authenticated);
    setUsername(r.username);
    return r;
  };

  const logout = async () => {
    await api.session.logout();
    setAuthenticated(false);
    setUsername(undefined);
  };

  return { authenticated, username, login, logout };
}
