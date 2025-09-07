import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { ChevronLeft, ChevronRight, CalendarSync, ChevronDownIcon } from "lucide-react";
import { useAuthStore } from "@/types/auth";
import { init, syncCalendar, updateHoursRange } from "@/lib/utils";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Calendar } from "@/components/ui/calendar";
import { Label } from "@/components/ui/label";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

export default function Header({ currentPage }: { currentPage: string }) {
  const { isLoggedIn, setLoggedIn } = useAuthStore();
  const [open, setOpen] = useState(false);
  const [selectedDate, setSelectedDate] = useState<Date | undefined>(undefined);

  // Hours and AM/PM for start and end times
  const [startHour, setStartHour] = useState<number>(12);
  const [startPeriod, setStartPeriod] = useState<"AM" | "PM">("PM");
  const [endHour, setEndHour] = useState<number>(5);
  const [endPeriod, setEndPeriod] = useState<"AM" | "PM">("PM");

  useEffect(() => {
    async function checkLogin() {
      const isValid = await init();
      setLoggedIn(isValid);
    }
    checkLogin();
  }, [setLoggedIn]);

  function to24Hour(hour12: number, period: "AM" | "PM"): number {
    if (period === "AM") {
      return hour12 === 12 ? 0 : hour12;
    }
    return hour12 === 12 ? 12 : hour12 + 12;
  }

  function handleConfirm() {
    if (!selectedDate) {
      console.warn("No date selected");
      return;
    }

    const startDate = new Date(selectedDate);
    const endDate = new Date(selectedDate);

    startDate.setHours(to24Hour(startHour, startPeriod), 0, 0, 0);
    endDate.setHours(to24Hour(endHour, endPeriod), 0, 0, 0);

    const meta = {
      date: selectedDate.toDateString(),
      start: startDate.toLocaleTimeString(),
      end: endDate.toLocaleTimeString(),
    };

    console.log("Sync parameters:", {
      ...meta,
      startISO: startDate.toISOString(),
      endISO: endDate.toISOString(),
    });

    // Fire backend command to summarize and push event for the range
    updateHoursRange(startDate.toISOString(), endDate.toISOString(), meta);

    setOpen(false);
  }

  async function handleLogout() {
    setLoggedIn(false);
  }

  const hoursOptions = Array.from({ length: 12 }, (_, i) => i + 1);

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
          <Dialog open={open} onOpenChange={setOpen}>
            <DialogTrigger asChild>
              <Button
                className="flex items-center gap-2 min-w-[84px] h-10 px-4 bg-blue-500 text-white text-sm font-semibold hover:bg-blue-600"
                onClick={() => {}}
              >
                <CalendarSync className="h-4 w-4" />
                <span className="truncate">Sync Calendar</span>
              </Button>
            </DialogTrigger>
            <DialogContent className="sm:max-w-[425px] bg-slate-800 text-white">
              <DialogHeader>
                <DialogTitle>Sync Calendar Event</DialogTitle>
              </DialogHeader>
              <div className="grid gap-4 py-4">
                <div className="flex flex-col gap-3">
                  <Label htmlFor="date-picker" className="px-1">Date</Label>
                  <Popover>
                    <PopoverTrigger asChild>
                      <Button
                        variant="outline"
                        id="date-picker"
                        className="w-32 justify-between font-normal bg-slate-700 border-gray-600"
                      >
                        {selectedDate ? selectedDate.toLocaleDateString() : "Select date"}
                        <ChevronDownIcon />
                      </Button>
                    </PopoverTrigger>
                    <PopoverContent className="p-0 bg-slate-700 border-gray-600 text-white" align="start">
                      <Calendar
                        mode="single"
                        selected={selectedDate}
                        onSelect={(date) => setSelectedDate(date)}
                        captionLayout="dropdown"
                        className="rounded-md border border-gray-600 w-full bg-slate-700 text-white"
                      />
                    </PopoverContent>
                  </Popover>
                </div>

                {/* Start and End Time on the same row */}
                <div className="flex gap-6">
                  {/* Start Time */}
                  <div className="flex flex-col gap-2">
                    <Label className="px-1 text-sm">Start</Label>
                    <div className="flex gap-2">
                      <Select value={startHour.toString()} onValueChange={(val) => setStartHour(parseInt(val))}>
                        <SelectTrigger className="w-20 bg-slate-700 border-gray-600 text-white hover:bg-slate-600">
                          <SelectValue placeholder="Hour" />
                        </SelectTrigger>
                        <SelectContent className="bg-slate-700 text-white border-gray-600">
                          {hoursOptions.map((hour) => (
                            <SelectItem key={hour} value={hour.toString()} className="hover:bg-slate-600">
                              {hour}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>

                      <Select value={startPeriod} onValueChange={(val) => setStartPeriod(val as "AM" | "PM")}>
                        <SelectTrigger className="w-20 bg-slate-700 border-gray-600 text-white hover:bg-slate-600">
                          <SelectValue placeholder="AM/PM" />
                        </SelectTrigger>
                        <SelectContent className="bg-slate-700 text-white border-gray-600">
                          <SelectItem value="AM" className="hover:bg-slate-600">AM</SelectItem>
                          <SelectItem value="PM" className="hover:bg-slate-600">PM</SelectItem>
                        </SelectContent>
                      </Select>
                    </div>
                  </div>

                  {/* End Time */}
                  <div className="flex flex-col gap-2">
                    <Label className="px-1 text-sm">End</Label>
                    <div className="flex gap-2">
                      <Select value={endHour.toString()} onValueChange={(val) => setEndHour(parseInt(val))}>
                        <SelectTrigger className="w-20 bg-slate-700 border-gray-600 text-white hover:bg-slate-600">
                          <SelectValue placeholder="Hour" />
                        </SelectTrigger>
                        <SelectContent className="bg-slate-700 text-white border-gray-600">
                          {hoursOptions.map((hour) => (
                            <SelectItem key={hour} value={hour.toString()} className="hover:bg-slate-600">
                              {hour}
                            </SelectItem>
                          ))}
                        </SelectContent>
                      </Select>

                      <Select value={endPeriod} onValueChange={(val) => setEndPeriod(val as "AM" | "PM")}>
                        <SelectTrigger className="w-20 bg-slate-700 border-gray-600 text-white hover:bg-slate-600">
                          <SelectValue placeholder="AM/PM" />
                        </SelectTrigger>
                        <SelectContent className="bg-slate-700 text-white border-gray-600">
                          <SelectItem value="AM" className="hover:bg-slate-600">AM</SelectItem>
                          <SelectItem value="PM" className="hover:bg-slate-600">PM</SelectItem>
                        </SelectContent>
                      </Select>
                    </div>
                  </div>
                </div>

              </div>

              <Button className="bg-blue-500 hover:bg-blue-600" onClick={handleConfirm}>
                Confirm Sync
              </Button>
            </DialogContent>
          </Dialog>
          <Button variant="outline" className="h-10 px-4 text-sm" onClick={handleLogout}>
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
