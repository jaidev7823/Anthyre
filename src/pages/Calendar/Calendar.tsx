import { useState } from 'react';
import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { X, CheckCircle } from 'lucide-react';

export default function Calendar() {
  const [selectedDate, setSelectedDate] = useState('Oct 2');

  const days = ['SUN', 'MON', 'TUE', 'WED', 'THU', 'FRI', 'SAT'];
  const dates = [
    { day: 29, prevMonth: true },
    { day: 30, prevMonth: true },
    { day: 1, progress: 70, color: '#4ade80' },
    { day: 2, progress: 90, color: '#4ade80', selected: true },
    { day: 3, progress: 50, color: '#facc15' },
    { day: 4, progress: 20, color: '#f87171' },
    { day: 5 },
    { day: 6 },
    { day: 7 },
    { day: 8 },
    { day: 9 },
    { day: 10 },
    { day: 11 },
    { day: 12 },
    { day: 13 },
    { day: 14 },
    { day: 15 },
    { day: 16 },
    { day: 17 },
    { day: 18 },
    { day: 19 },
    { day: 20 },
    { day: 21 },
    { day: 22 },
    { day: 23 },
    { day: 24 },
    { day: 25 },
    { day: 26 },
    { day: 27 },
    { day: 28 },
    { day: 29 },
    { day: 30 },
    { day: 31 },
    { day: 1, nextMonth: true },
    { day: 2, nextMonth: true },
  ];

  return (
    <div className="flex-1 flex flex-col">
      <div className="flex-1 flex overflow-hidden">
        <div className="flex-1 p-6 overflow-y-auto">
          <Card className="border-gray-800 rounded-2xl overflow-hidden">
            <CardContent className="p-0 grid grid-cols-7 gap-px bg-gray-800">
              {days.map((day) => (
                <div
                  key={day}
                  className="text-center py-3 bg-slate-900 text-sm font-medium text-gray-400 border border-gray-800"
                >
                  {day}
                </div>
              ))}
              {dates.map((date, index) => (
                <div
                  key={index}
                  className={`h-36 p-2 bg-slate-900 border border-gray-800 flex flex-col justify-between ${
                    date.prevMonth || date.nextMonth
                      ? 'text-gray-600'
                      : 'hover:bg-gray-800 cursor-pointer transition-colors'
                  } ${date.selected ? 'border-2 border-blue-500 -m-px rounded-lg' : ''}`}
                  onClick={() => !date.prevMonth && !date.nextMonth && setSelectedDate(`Oct ${date.day}`)}
                >
                  <span
                    className={`font-medium ${date.selected ? 'text-blue-500 font-bold' : ''}`}
                  >
                    {date.day}
                  </span>
                  {date.progress && (
                    <div className="self-end">
                      <div
                        className="progress-circle"
                        style={{ '--progress': date.progress, '--color': date.color } as any}
                      ></div>
                    </div>
                  )}
                </div>
              ))}
            </CardContent>
          </Card>
        </div>
        <aside className="w-96 flex-shrink-0 bg-slate-900 border border-gray-800 p-6 flex flex-col gap-6 overflow-y-auto">
          <div className="flex items-center justify-between">
            <h2 className="text-xl font-semibold text-white">Day Detail - {selectedDate}</h2>
            <Button
              variant="ghost"
              size="icon"
              className="p-2 text-gray-400 hover:bg-gray-800"
              onClick={() => setSelectedDate('')}
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