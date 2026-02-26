import { Link, useLocation } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { useAuth } from '../hooks/useAuth';

export default function Navbar() {
  const { t } = useTranslation();
  const { username, logout } = useAuth();
  const { pathname } = useLocation();

  const navLink = (to: string, label: string) => (
    <Link
      to={to}
      className={`text-sm px-3 py-1 rounded ${
        pathname === to
          ? 'bg-gray-700 text-white'
          : 'text-gray-400 hover:text-white'
      }`}
    >
      {label}
    </Link>
  );

  return (
    <nav className="bg-gray-900 border-b border-gray-700 px-6 py-3 flex items-center justify-between">
      <div className="flex items-center gap-4">
        <span className="text-white font-bold text-lg">wg-easy</span>
        {navLink('/', t('nav.dashboard'))}
        {navLink('/settings', t('nav.settings'))}
      </div>
      <div className="flex items-center gap-3">
        {username && <span className="text-gray-400 text-sm">{username}</span>}
        <button
          onClick={logout}
          className="text-sm text-gray-400 hover:text-white"
        >
          {t('nav.logout')}
        </button>
      </div>
    </nav>
  );
}
