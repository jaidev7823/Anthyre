import { useEffect } from "react";
import { Button } from "@/components/ui/button";
import { ChevronLeft, ChevronRight, CalendarSync } from "lucide-react";
import { useAuthStore } from "@/types/auth";
import { init, syncCalendar, updateHours } from "@/lib/utils";

export default function Header({ currentPage }: { currentPage: string }) {
  const { isLoggedIn, setLoggedIn } = useAuthStore();

  // 🔑 Automatically run when component mounts
  useEffect(() => {
    async function checkLogin() {
      const isValid = await init();
      setLoggedIn(isValid);
    }
    checkLogin();
  }, [setLoggedIn]);

  async function handleLogin() {
    const isValid = await init();
    if (isValid) {
      setLoggedIn(true);
    } else {
      setLoggedIn(false);
      // TODO: launch Google login if needed
    }
  }

  async function handleLogout() {
    setLoggedIn(false);
    // TODO: optionally clear tokens in DB
  }

  return (
    <header className="flex items-center justify-between border-b border-gray-800 px-6 py-4 bg-slate-900">
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="icon"
            className="p-2 text-gray-400 hover:text-white hover:bg-gray-800"
            onClick={() => console.log("Previous clicked")}
          >
            <ChevronLeft className="h-6 w-6" />
          </Button>
          <h2 className="text-xl font-semibold text-white">
            {currentPage === "calendar" ? "October 2023" : "June 12, 2024"}
          </h2>
          <Button
            variant="ghost"
            size="icon"
            className="p-2 text-gray-400 hover:text-white hover:bg-gray-800"
            onClick={() => console.log("Next clicked")}
          >
            <ChevronRight className="h-6 w-6" />
          </Button>
        </div>
      </div>

      {isLoggedIn ? (
        <div className="flex gap-2">
          <Button
            className="flex items-center gap-2 min-w-[84px] h-10 px-4 bg-blue-500 text-white text-sm font-semibold hover:bg-blue-600"
            onClick={updateHours}
          >
            <CalendarSync className="h-4 w-4" />
            <span className="truncate">Sync Calendar</span>
          </Button>
          <Button
            variant="outline"
            className="h-10 px-4 text-sm"
            onClick={handleLogout}
          >
            Logout
          </Button>
        </div>
      ) : (
        <Button
          className="flex items-center gap-2 min-w-[84px] h-10 px-4 bg-green-500 text-white text-sm font-semibold hover:bg-green-600"
          onClick={syncCalendar}
        >
          Login with Google
        </Button>
      )}
    </header>
  );
}
