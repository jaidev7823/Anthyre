import { useMemo, useState } from 'react';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { X, CheckCircle, ChevronLeft, ChevronRight } from 'lucide-react';

export default function Calendar() {
  const today = useMemo(() => new Date(), []);
  const [visibleMonth, setVisibleMonth] = useState<Date>(
    new Date(today.getFullYear(), today.getMonth(), 1)
  );
  const [selectedDate, setSelectedDate] = useState<Date | null>(today);

  const dayLabels = useMemo(() => {
    const base = new Date(2021, 7, 1); // Sunday
    return [...Array(7)].map((_, i) =>
      new Date(base.getFullYear(), base.getMonth(), base.getDate() + i)
        .toLocaleDateString(undefined, { weekday: 'short' })
        .toUpperCase()
    );
  }, []);

  function startOfMonth(d: Date) {
    return new Date(d.getFullYear(), d.getMonth(), 1);
  }

  function endOfMonth(d: Date) {
    return new Date(d.getFullYear(), d.getMonth() + 1, 0);
  }

  const monthGrid = useMemo(() => {
    const start = startOfMonth(visibleMonth);
    const end = endOfMonth(visibleMonth);
    const startWeekday = start.getDay();

    const daysInPrevMonth = new Date(visibleMonth.getFullYear(), visibleMonth.getMonth(), 0).getDate();
    const daysInThisMonth = end.getDate();

    const cells: Array<{ date: Date; inMonth: boolean }> = [];

    // Leading days from previous month
    for (let i = startWeekday - 1; i >= 0; i--) {
      const day = daysInPrevMonth - i;
      cells.push({
        date: new Date(visibleMonth.getFullYear(), visibleMonth.getMonth() - 1, day),
        inMonth: false,
      });
    }

    // Current month days
    for (let d = 1; d <= daysInThisMonth; d++) {
      cells.push({
        date: new Date(visibleMonth.getFullYear(), visibleMonth.getMonth(), d),
        inMonth: true,
      });
    }

    // Trailing days from next month to complete 6 rows (42 cells)
    const remaining = 42 - cells.length;
    for (let d = 1; d <= remaining; d++) {
      cells.push({
        date: new Date(visibleMonth.getFullYear(), visibleMonth.getMonth() + 1, d),
        inMonth: false,
      });
    }

    return cells;
  }, [visibleMonth]);

  function isSameDay(a: Date | null, b: Date) {
    if (!a) return false;
    return a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate();
  }

  function colorForProgress(pct: number): string {
    if (pct >= 80) return 'bg-emerald-500';
    if (pct >= 50) return 'bg-amber-400';
    return 'bg-rose-500';
  }

  // Placeholder progress: you can wire real data here later
  function getProgressForDate(_date: Date): number | null {
    return null;
  }

  const title = useMemo(
    () => visibleMonth.toLocaleDateString(undefined, { month: 'long', year: 'numeric' }),
    [visibleMonth]
  );

  return (
    <div className="flex-1 flex flex-col">
      <div className="flex-1 flex overflow-hidden">
        <div className="flex-1 p-4 sm:p-6 overflow-y-auto">
          <div className="mb-4 flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Button
                variant="ghost"
                size="icon"
                className="text-gray-300 hover:bg-gray-800"
                onClick={() =>
                  setVisibleMonth((m) => new Date(m.getFullYear(), m.getMonth() - 1, 1))
                }
                aria-label="Previous month"
              >
                <ChevronLeft className="h-5 w-5" />
              </Button>
              <Button
                variant="ghost"
                size="icon"
                className="text-gray-300 hover:bg-gray-800"
                onClick={() => setVisibleMonth(new Date(today.getFullYear(), today.getMonth(), 1))}
                aria-label="Today"
              >
                <span className="text-sm">Today</span>
              </Button>
              <Button
                variant="ghost"
                size="icon"
                className="text-gray-300 hover:bg-gray-800"
                onClick={() =>
                  setVisibleMonth((m) => new Date(m.getFullYear(), m.getMonth() + 1, 1))
                }
                aria-label="Next month"
              >
                <ChevronRight className="h-5 w-5" />
              </Button>
            </div>
            <h2 className="text-lg sm:text-xl font-semibold text-white">{title}</h2>
            <div className="w-24" />
          </div>

          <Card className="border-gray-800 rounded-2xl">
            <CardContent className="p-0">
              <div className="grid grid-cols-7 gap-px bg-gray-800">
                {dayLabels.map((label) => (
                  <div
                    key={label}
                    className="text-center py-2 sm:py-3 bg-slate-900 text-xs sm:text-sm font-medium text-gray-400 border border-gray-800"
                  >
                    {label}
                  </div>
                ))}

                {monthGrid.map((cell, index) => {``
                  const dayNum = cell.date.getDate();
                  const inMonth = cell.inMonth;
                  const selected = isSameDay(selectedDate, cell.date);
                  const progress = getProgressForDate(cell.date);

                  return (
                    <div
                      key={`${cell.date.toISOString()}-${index}`}
                      className={`h-24 sm:h-32 p-2 bg-slate-900 border border-gray-800 flex flex-col gap-2 ${
                        inMonth
                          ? 'hover:bg-gray-800 cursor-pointer transition-colors'
                          : 'text-gray-600'
                      } ${selected ? 'relative z-10 ring-2 ring-blue-500 -m-px rounded-lg' : ''}`}
                      onClick={() => inMonth && setSelectedDate(cell.date)}
                    >
                      <div className="flex items-center justify-between">
                        <span className={`text-sm font-medium ${
                          selected ? 'text-blue-400 font-semibold' : inMonth ? 'text-white' : 'text-gray-500'
                        }`}>
                          {dayNum}
                        </span>
                        {isSameDay(today, cell.date) && (
                          <span className="text-[10px] px-1.5 py-0.5 rounded bg-blue-500/20 text-blue-300">Today</span>
                        )}
                      </div>

                      {typeof progress === 'number' && (
                        <div className="mt-auto">
                          <div className="w-full h-1.5 rounded-full bg-gray-700 overflow-hidden">
                            <div
                              className={`h-full ${colorForProgress(progress)}`}
                              style={{ width: `${Math.min(100, Math.max(0, progress))}%` }}
                            />
                          </div>
                          <div className="mt-1 text-[10px] text-gray-400 text-right">{progress}%</div>
                        </div>
                      )}
                    </div>
                  );
                })}
              </div>
            </CardContent>
          </Card>
        </div>
        <aside className="hidden md:flex w-96 flex-shrink-0 bg-slate-900 border border-gray-800 p-6 flex-col gap-6 overflow-y-auto">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold text-white">
              Day Detail {selectedDate ? `- ${selectedDate.toLocaleDateString()}` : ''}
            </h2>
            <Button
              variant="ghost"
              size="icon"
              className="p-2 text-gray-400 hover:bg-gray-800"
              onClick={() => setSelectedDate(null)}
            >
              <X className="h-5 w-5" />
            </Button>
          </div>
          <Card className="bg-gray-800/50 border-gray-800">
            <CardHeader className="flex flex-row justify-between items-center pt-6 space-y-0">
              <h3 className="text-lg font-semibold text-white">AI Reflection Summary</h3>
            </CardHeader>
            <CardContent className="pb-6">
              <p className="text-sm text-gray-400">
                Today was a productive day with most tasks completed as planned. However, there were some deviations in the afternoon due to unexpected meetings. Focus on time management and prioritizing tasks for tomorrow.
              </p>
            </CardContent>
          </Card>
          <Card className="bg-gray-800/50 border-gray-800">
            <CardHeader className="flex flex-row justify-between items-center pt-6 space-y-0">
              <h3 className="text-lg font-semibold text-white">Planned vs Actual</h3>
            </CardHeader>
            <CardContent className="pb-6">
              <div className="space-y-3">
                <div className="flex justify-between items-center text-sm">
                  <p className="text-gray-400">Planned Tasks</p>
                  <p className="font-medium text-white">8</p>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <p className="text-gray-400">Actual Tasks Completed</p>
                  <p className="font-medium text-white">6</p>
                </div>
                <div className="flex justify-between items-center text-sm">
                  <p className="text-gray-400">Difference</p>
                  <p className="font-medium text-red-400">-2</p>
                </div>
              </div>
            </CardContent>
          </Card>
          <Card className="bg-gray-800/50 border-gray-800">
            <CardHeader className="flex flex-row justify-between items-center pt-6 space-y-0">
              <h3 className="text-lg font-semibold text-white">Suggestions for Improvement</h3>
            </CardHeader>
            <CardContent className="pb-6">
              <ul className="space-y-3">
                <li className="flex items-start gap-3 text-sm text-gray-300">
                  <CheckCircle className="h-5 w-5 text-green-400 mt-0.5" />
                  <span>Review daily schedule in the morning to set clear intentions.</span>
                </li>
                <li className="flex items-start gap-3 text-sm text-gray-300">
                  <CheckCircle className="h-5 w-5 text-green-400 mt-0.5" />
                  <span>Allocate buffer time between tasks for unexpected events.</span>
                </li>
                <li className="flex items-start gap-3 text-sm text-gray-300">
                  <CheckCircle className="h-5 w-5 text-green-400 mt-0.5" />
                  <span>Prioritize tasks using a framework like the Eisenhower Matrix.</span>
                </li>
              </ul>
            </CardContent>
          </Card>
        </aside>
      </div>
    </div>
  );
}