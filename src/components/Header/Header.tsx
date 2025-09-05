import { Button } from '@/components/ui/button';
import { ChevronLeft, ChevronRight, CalendarSync } from 'lucide-react';
import { syncCalendar } from '@/lib/utils';

export default function Header({ currentPage }: { currentPage: string }) {
  return (
    <header className="flex items-center justify-between border-b border-gray-800 px-6 py-4 bg-slate-900">
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
          <Button
            variant="ghost"
            size="icon"
            className="p-2 text-gray-400 hover:text-white hover:bg-gray-800"
            onClick={() => console.log('Previous clicked')}
          >
            <ChevronLeft className="h-6 w-6" />
          </Button>
          <h2 className="text-xl font-semibold text-white">
            {currentPage === 'calendar' ? 'October 2023' : 'June 12, 2024'}
          </h2>
          <Button
            variant="ghost"
            size="icon"
            className="p-2 text-gray-400 hover:text-white hover:bg-gray-800"
            onClick={() => console.log('Next clicked')}
          >
            <ChevronRight className="h-6 w-6" />
          </Button>
        </div>
      </div>
      <Button
        className="flex items-center gap-2 min-w-[84px] h-10 px-4 bg-blue-500 text-white text-sm font-semibold hover:bg-blue-600"
        onClick={async () => {
          await syncCalendar();
        }}
      >
        <CalendarSync className="h-4 w-4" />
        <span className="truncate">Sync Calendar</span>
      </Button>
    </header>
  );
}