import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useClients } from '../hooks/useClients';
import { useStats } from '../hooks/useStats';
import ClientTable from '../components/ClientTable';
import ClientModal from '../components/ClientModal';
import type { Client } from '../api/types';

export default function DashboardPage() {
  const { t } = useTranslation();
  const { data: clients = [], isLoading, create } = useClients();
  const { data: stats = [] } = useStats();
  const [selectedClient, setSelectedClient] = useState<Client | null>(null);
  const [newClientName, setNewClientName] = useState('');
  const [creating, setCreating] = useState(false);

  const statsMap = Object.fromEntries(stats.map((s) => [s.public_key, s]));

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newClientName.trim()) return;
    setCreating(true);
    try {
      await create.mutateAsync({ name: newClientName.trim() });
      setNewClientName('');
    } finally {
      setCreating(false);
    }
  };

  return (
    <div className="p-6">
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-semibold text-white">{t('dashboard.clients')}</h2>
        <form onSubmit={handleCreate} className="flex gap-2">
          <input
            type="text"
            value={newClientName}
            onChange={(e) => setNewClientName(e.target.value)}
            placeholder={t('dashboard.newClientName')}
            className="bg-gray-700 text-white rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            type="submit"
            disabled={creating || !newClientName.trim()}
            className="bg-blue-600 hover:bg-blue-700 text-white text-sm px-4 py-2 rounded disabled:opacity-50"
          >
            {creating ? t('dashboard.creating') : t('dashboard.addClient')}
          </button>
        </form>
      </div>

      {isLoading ? (
        <p className="text-gray-400">{t('common.loading')}</p>
      ) : (
        <ClientTable
          clients={clients}
          statsMap={statsMap}
          onSelect={setSelectedClient}
        />
      )}

      {selectedClient && (
        <ClientModal
          client={selectedClient}
          onClose={() => setSelectedClient(null)}
        />
      )}
    </div>
  );
}
