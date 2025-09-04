import { Button } from '@/components/ui/button';
import { Avatar, AvatarImage, AvatarFallback } from '@/components/ui/avatar';
import { ChevronLeft, ChevronRight, CalendarSync } from 'lucide-react';
import { syncCalendar } from '@/lib/utils';

export default function Header() {
  return (
    <header className="flex items-center justify-between whitespace-nowrap border-b border-gray-800 px-8 py-4 bg-slate-900">
      <div className="flex items-center gap-4">
        <Button
          variant="ghost"
          size="icon"
          className="p-2 text-gray-400 hover:text-white hover:bg-gray-800"
          onClick={() => console.log('Previous date clicked')}
        >
          <ChevronLeft className="h-6 w-6" />
        </Button>
        <span className="truncate text-white text-sm">June 12, 2024</span>
        <Button
          variant="ghost"
          size="icon"
          className="p-2 text-gray-400 hover:text-white hover:bg-gray-800"
          onClick={() => console.log('Next date clicked')}
        >
          <ChevronRight className="h-6 w-6" />
        </Button>
      </div>
      <div className="flex items-center gap-4">
        <Button
          className="flex items-center gap-2 min-w-[84px] h-10 px-4 bg-gray-800 text-white text-sm font-medium hover:bg-gray-700"
          onClick={async () => {
            await syncCalendar();
          }}
        >
          <CalendarSync className="h-4 w-4" />
          <span className="truncate">Sync Calendar</span>
        </Button>
        <Avatar className="size-10">
          <AvatarImage
            src="https://lh3.googleusercontent.com/aida-public/AB6AXuCoKctPkFCLX8WyetwGosy94Qn2kwTSmJfz5pHLoYq52azQBuNrr57jmR5fqEnwTiyqbWlXjPbpOIpGiNqvPi8QI1s3vvejAHXgcChgz8nEcDLTm8D6JwlT1rPWwCCCToBrp74tJpXXTrkYzxklf-F4TKzPEmFc2hLyX_0wGNleXf5l57vIo2uCnyzRm4HSu2PNzG4xdEat72NRfxdbsalAEW-zsDOxzGn1wJWLvdqawMWYY1EopjYuZNuv7jZyBBC4q-7EmWYlf-qT"
            alt="User avatar"
          />
          <AvatarFallback>JD</AvatarFallback>
        </Avatar>
      </div>
    </header>
  );
}