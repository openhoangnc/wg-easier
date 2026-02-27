import { useTranslation } from 'react-i18next';
import { useClients } from '../hooks/useClients';
import type { Client, PeerStats } from '../api/types';

interface ClientTableProps {
  clients: Client[];
  statsMap: Record<string, PeerStats>;
  onSelect: (client: Client) => void;
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
  return `${(bytes / 1024 / 1024 / 1024).toFixed(2)} GB`;
}

function isOnline(lastHandshakeSecs?: number): boolean {
  if (!lastHandshakeSecs) return false;
  const now = Date.now() / 1000;
  return now - lastHandshakeSecs < 150; // 2.5 minutes
}

export default function ClientTable({ clients, statsMap, onSelect }: ClientTableProps) {
  const { t } = useTranslation();
  const { enable, disable, remove } = useClients();

  if (clients.length === 0) {
    return (
      <div className="text-center text-gray-500 py-16">
        <p>{t('dashboard.noClients')}</p>
      </div>
    );
  }

  return (
    <div className="overflow-x-auto">
      <table className="w-full text-sm text-left text-gray-300">
        <thead className="text-xs uppercase text-gray-500 border-b border-gray-700">
          <tr>
            <th className="pb-3 pr-4">{t('client.name')}</th>
            <th className="pb-3 pr-4">{t('client.ip')}</th>
            <th className="pb-3 pr-4">{t('client.status')}</th>
            <th className="pb-3 pr-4">{t('client.lastHandshake')}</th>
            <th className="pb-3 pr-4">{t('client.transfer')}</th>
            <th className="pb-3">{t('client.actions')}</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-gray-800">
          {clients.map((client) => {
            const stats = statsMap[client.public_key];
            const online = isOnline(stats?.last_handshake_secs);
            return (
              <tr key={client.id} className="hover:bg-gray-800/50">
                <td className="py-3 pr-4 font-medium text-white">{client.name}</td>
                <td className="py-3 pr-4 font-mono text-xs">{client.ipv4}</td>
                <td className="py-3 pr-4">
                  <span
                    className={`inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full ${
                      online ? 'bg-green-900 text-green-400' : 'bg-gray-700 text-gray-400'
                    }`}
                  >
                    <span className={`w-1.5 h-1.5 rounded-full ${online ? 'bg-green-400' : 'bg-gray-500'}`} />
                    {online ? t('client.online') : t('client.offline')}
                  </span>
                </td>
                <td className="py-3 pr-4 text-xs text-gray-400">
                  {stats?.last_handshake_secs
                    ? new Date(stats.last_handshake_secs * 1000).toLocaleString()
                    : '—'}
                </td>
                <td className="py-3 pr-4 text-xs">
                  {stats ? (
                    <span>↑ {formatBytes(stats.tx_bytes)} ↓ {formatBytes(stats.rx_bytes)}</span>
                  ) : '—'}
                </td>
                <td className="py-3">
                  <div className="flex items-center gap-2">
                    <button
                      onClick={() => onSelect(client)}
                      className="text-blue-400 hover:text-blue-300 text-xs"
                    >
                      {t('client.details')}
                    </button>
                    {client.enabled ? (
                      <button
                        onClick={() => disable.mutate(client.id)}
                        className="text-yellow-400 hover:text-yellow-300 text-xs"
                      >
                        {t('client.disable')}
                      </button>
                    ) : (
                      <button
                        onClick={() => enable.mutate(client.id)}
                        className="text-green-400 hover:text-green-300 text-xs"
                      >
                        {t('client.enable')}
                      </button>
                    )}
                    <button
                      onClick={() => {
                        if (confirm(t('client.confirmDelete'))) {
                          remove.mutate(client.id);
                        }
                      }}
                      className="text-red-400 hover:text-red-300 text-xs"
                    >
                      {t('client.delete')}
                    </button>
                  </div>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
