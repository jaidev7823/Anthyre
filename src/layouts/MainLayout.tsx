import { Outlet, useLocation } from 'react-router-dom';
import Sidebar from '../components/Sidebar/Sidebar';
import Header from '../components/Header/Header';

export default function MainLayout() {
  const location = useLocation();
  const currentPage = location.pathname.slice(1) || 'today';

  return (
    <div className="flex h-screen">
      <Sidebar />
      <div className="flex-1 flex flex-col overflow-hidden">
        <Header currentPage={currentPage} />
        <main className="flex-1 overflow-y-auto p-8 bg-slate-900">
          <Outlet />
        </main>
      </div>
    </div>
  );
}