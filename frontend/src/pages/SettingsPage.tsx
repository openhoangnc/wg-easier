import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useInterface } from '../hooks/useInterface';
import { api } from '../api/client';

export default function SettingsPage() {
  const { t } = useTranslation();
  const { data: iface, isLoading } = useInterface();
  const [listenPort, setListenPort] = useState('');
  const [ipv4Cidr, setIpv4Cidr] = useState('');
  const [saved, setSaved] = useState(false);

  const handleSave = async (e: React.FormEvent) => {
    e.preventDefault();
    await api.interface.update({
      listen_port: listenPort ? Number(listenPort) : undefined,
      ipv4_cidr: ipv4Cidr || undefined,
    });
    setSaved(true);
    setTimeout(() => setSaved(false), 2000);
  };

  if (isLoading) return <p className="text-gray-400 p-6">{t('common.loading')}</p>;

  return (
    <div className="p-6 max-w-lg">
      <h2 className="text-xl font-semibold text-white mb-6">{t('settings.title')}</h2>

      <div className="bg-gray-800 rounded-lg p-4 mb-6">
        <h3 className="text-gray-300 font-medium mb-3">{t('settings.interfaceInfo')}</h3>
        <dl className="space-y-2 text-sm">
          <div className="flex justify-between">
            <dt className="text-gray-400">{t('settings.publicKey')}</dt>
            <dd className="text-gray-200 font-mono text-xs truncate max-w-xs">{iface?.public_key}</dd>
          </div>
          <div className="flex justify-between">
            <dt className="text-gray-400">{t('settings.listenPort')}</dt>
            <dd className="text-gray-200">{iface?.listen_port}</dd>
          </div>
          <div className="flex justify-between">
            <dt className="text-gray-400">{t('settings.subnet')}</dt>
            <dd className="text-gray-200">{iface?.ipv4_cidr}</dd>
          </div>
        </dl>
      </div>

      <form onSubmit={handleSave} className="bg-gray-800 rounded-lg p-4 space-y-4">
        <h3 className="text-gray-300 font-medium">{t('settings.updateInterface')}</h3>
        <div>
          <label className="block text-sm text-gray-400 mb-1">{t('settings.listenPort')}</label>
          <input
            type="number"
            value={listenPort}
            onChange={(e) => setListenPort(e.target.value)}
            placeholder={String(iface?.listen_port ?? 51820)}
            className="w-full bg-gray-700 text-white rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div>
          <label className="block text-sm text-gray-400 mb-1">{t('settings.subnet')}</label>
          <input
            type="text"
            value={ipv4Cidr}
            onChange={(e) => setIpv4Cidr(e.target.value)}
            placeholder={iface?.ipv4_cidr ?? '10.8.0.0/24'}
            className="w-full bg-gray-700 text-white rounded px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <button
          type="submit"
          className="bg-blue-600 hover:bg-blue-700 text-white text-sm px-4 py-2 rounded"
        >
          {saved ? t('common.saved') : t('common.save')}
        </button>
      </form>
    </div>
  );
}
