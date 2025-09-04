import { Calendar, ChartBar,ListTodo , Settings } from 'lucide-react';

export default function Sidebar() {
  return (
    <aside className="w-64 bg-slate-900 p-6 flex flex-col justify-between border-r border-gray-800">
      <div>
        <h1 className="text-2xl font-bold mb-8">Anthyre</h1>
        <nav className="flex flex-col gap-2">
          <a
            href="#"
            className="flex items-center gap-3 px-3 py-2 rounded-md bg-gray-800 text-white"
          >
            <ListTodo className="h-5 w-5" />
            <span>Today</span>
          </a>
          <a
            href="#"
            className="flex items-center gap-3 px-3 py-2 rounded-md text-gray-400 hover:bg-gray-800"
          >
            <Calendar className="h-5 w-5" />
            <span>Calendar</span>
          </a>
          <a
            href="#"
            className="flex items-center gap-3 px-3 py-2 rounded-md text-gray-400 hover:bg-gray-800"
          >
            <ChartBar className="h-5 w-5" />
            <span>Reports</span>
          </a>
        </nav>
      </div>
      <div>
        <a
          href="#"
          className="flex items-center gap-3 px-3 py-2 rounded-md text-gray-400 hover:bg-gray-800"
        >
          <Settings className="h-5 w-5" />
          <span>Settings</span>
        </a>
      </div>
    </aside>
  );
}