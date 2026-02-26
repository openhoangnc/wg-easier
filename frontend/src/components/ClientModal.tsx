import { useTranslation } from 'react-i18next';
import { api } from '../api/client';
import type { Client } from '../api/types';

interface ClientModalProps {
  client: Client;
  onClose: () => void;
}

export default function ClientModal({ client, onClose }: ClientModalProps) {
  const { t } = useTranslation();

  return (
    <div className="fixed inset-0 bg-black/60 flex items-center justify-center z-50" onClick={onClose}>
      <div
        className="bg-gray-800 rounded-lg p-6 w-full max-w-md shadow-xl"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-white">{client.name}</h3>
          <button onClick={onClose} className="text-gray-400 hover:text-white text-xl">&times;</button>
        </div>

        <div className="mb-4 flex justify-center">
          <img
            src={api.clients.qrcodeUrl(client.id)}
            alt="QR Code"
            className="w-48 h-48"
          />
        </div>

        <div className="space-y-2 text-sm text-gray-300 mb-4">
          <div className="flex justify-between">
            <span className="text-gray-400">{t('client.ip')}</span>
            <span className="font-mono">{client.ipv4}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-400">{t('client.publicKey')}</span>
            <span className="font-mono text-xs truncate max-w-xs">{client.public_key}</span>
          </div>
          <div className="flex justify-between">
            <span className="text-gray-400">{t('client.status')}</span>
            <span className={client.enabled ? 'text-green-400' : 'text-gray-400'}>
              {client.enabled ? t('client.enabled') : t('client.disabled')}
            </span>
          </div>
        </div>

        <a
          href={api.clients.confUrl(client.id)}
          download
          className="block w-full text-center bg-blue-600 hover:bg-blue-700 text-white text-sm py-2 rounded mb-2"
        >
          {t('client.downloadConf')}
        </a>
        <button
          onClick={onClose}
          className="block w-full text-center bg-gray-700 hover:bg-gray-600 text-white text-sm py-2 rounded"
        >
          {t('common.close')}
        </button>
      </div>
    </div>
  );
}
