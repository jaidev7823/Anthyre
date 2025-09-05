import { NavLink } from "react-router-dom";
import { Calendar, ChartBar, Settings } from "lucide-react";

export default function Sidebar() {
  return (
    <aside className="w-64 bg-slate-900 p-6 flex flex-col justify-between border-r border-gray-800">
      <div>
        <h1 className="text-2xl font-bold mb-8 text-white">Anthyre</h1>
        <nav className="flex flex-col gap-2">
          <NavLink
            to="/"
            className={({ isActive }) =>
              `flex items-center gap-3 px-3 py-2 rounded-md text-white ${
                isActive ? "bg-gray-800" : "text-gray-400 hover:bg-gray-800"
              }`
            }
          >
            <Calendar className="h-5 w-5" />
            <span>Today</span>
          </NavLink>
          <NavLink
            to="/calendar"
            className={({ isActive }) =>
              `flex items-center gap-3 px-3 py-2 rounded-md text-white ${
                isActive ? "bg-gray-800" : "text-gray-400 hover:bg-gray-800"
              }`
            }
          >
            <Calendar className="h-5 w-5" />
            <span>Calendar</span>
          </NavLink>
          <NavLink
            to="/reports"
            className={({ isActive }) =>
              `flex items-center gap-3 px-3 py-2 rounded-md text-white ${
                isActive ? "bg-gray-800" : "text-gray-400 hover:bg-gray-800"
              }`
            }
          >
            <ChartBar className="h-5 w-5" />
            <span>Reports</span>
          </NavLink>
        </nav>
      </div>
      <div>
        <NavLink
          to="/settings"
          className={({ isActive }) =>
            `flex items-center gap-3 px-3 py-2 rounded-md text-white ${
              isActive ? "bg-gray-800" : "text-gray-400 hover:bg-gray-800"
            }`
          }
        >
          <Settings className="h-5 w-5" />
          <span>Settings</span>
        </NavLink>
      </div>
    </aside>
  );
}
