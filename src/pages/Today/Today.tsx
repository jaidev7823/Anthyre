import { Card, CardHeader, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { RefreshCw, AlertTriangle, XCircle, CheckCircle } from 'lucide-react';
import { syncCalendar } from '@/lib/utils';

export default function Today() {
  return (
    <div className="grid grid-cols-3 gap-8">
      {/* Main Body (Unchanged) */}
      <div className="col-span-2 space-y-4">
        {/* Time Block: 8:00 AM - 9:00 AM */}
        <Card className="border-gray-800 bg-slate-900">
          <CardHeader className="px-4 py-3 bg-gray-800/50 flex justify-between items-center">
            <h3 className="font-semibold text-white">8:00 AM - 9:00 AM</h3>
          </CardHeader>
          <CardContent className="p-4 grid grid-cols-2 gap-4">
            <div>
              <h4 className="font-medium text-gray-400 mb-2">Planned</h4>
              <p className="p-3 bg-gray-800 rounded-md text-white">Morning Routine</p>
            </div>
            <div>
              <h4 className="font-medium text-gray-400 mb-2">Actual</h4>
              <p className="p-3 bg-gray-800 rounded-md text-white">Morning Routine</p>
            </div>
          </CardContent>
        </Card>

        {/* Time Block: 9:00 AM - 12:00 PM */}
        <Card className="border-gray-800 bg-slate-900">
          <CardHeader className="px-4 py-3 bg-gray-800/50 flex justify-between items-center">
            <h3 className="font-semibold text-white">9:00 AM - 12:00 PM</h3>
          </CardHeader>
          <CardContent className="p-4 grid grid-cols-2 gap-4">
            <div>
              <h4 className="font-medium text-gray-400 mb-2">Planned</h4>
              <div className="space-y-2">
                <p className="p-3 bg-gray-800 rounded-md text-white">Work on Project X</p>
                <p className="p-3 bg-gray-800 rounded-md text-white">Meeting with Team</p>
                <p className="p-3 bg-gray-800 rounded-md text-white">Client Call</p>
              </div>
            </div>
            <div>
              <h4 className="font-medium text-gray-400 mb-2">Actual</h4>
              <div className="space-y-2">
                <p className="p-3 bg-gray-800 rounded-md text-white">Work on Project X</p>
                <p className="p-3 bg-gray-800 rounded-md text-white">Meeting with Team</p>
                <p className="p-3 bg-gray-800 rounded-md text-white">Client Call - Ran 15 mins over</p>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Time Block: 12:00 PM - 1:00 PM */}
        <Card className="border-gray-800 bg-slate-900">
          <CardHeader className="px-4 py-3 bg-gray-800/50 flex justify-between items-center">
            <h3 className="font-semibold text-white">12:00 PM - 1:00 PM</h3>
          </CardHeader>
          <CardContent className="p-4 grid grid-cols-2 gap-4">
            <div>
              <h4 className="font-medium text-gray-400 mb-2">Planned</h4>
              <p className="p-3 bg-gray-800 rounded-md text-white">Lunch Break</p>
            </div>
            <div>
              <h4 className="font-medium text-gray-400 mb-2">Actual</h4>
              <p className="p-3 bg-gray-800 rounded-md text-white">Lunch Break</p>
            </div>
          </CardContent>
        </Card>

        {/* Time Block: 1:00 PM - 5:00 PM */}
        <Card className="border-gray-800 bg-slate-900">
          <CardHeader className="px-4 py-3 bg-gray-800/50 flex justify-between items-center">
            <h3 className="font-semibold text-white">1:00 PM - 5:00 PM</h3>
          </CardHeader>
          <CardContent className="p-4 grid grid-cols-2 gap-4">
            <div>
              <h4 className="font-medium text-gray-400 mb-2">Planned</h4>
              <div className="space-y-2">
                <p className="p-3 bg-gray-800 rounded-md text-white">Research</p>
                <p className="p-3 bg-gray-800 rounded-md text-white">Documentation</p>
                <p className="p-3 bg-gray-800 rounded-md text-white">Follow-up Emails</p>
                <p className="p-3 bg-gray-800 rounded-md text-white">Wrap Up</p>
              </div>
            </div>
            <div>
              <h4 className="font-medium text-gray-400 mb-2">Actual</h4>
              <div className="space-y-2">
                <p className="p-3 bg-gray-800 rounded-md text-white">Research</p>
                <p className="p-3 bg-gray-800 rounded-md text-white">Documentation</p>
                <p className="p-3 bg-red-900/50 border border-red-700 rounded-md text-white">Got distracted by social media</p>
                <p className="p-3 bg-gray-800 rounded-md text-white">Follow-up Emails</p>
                <p className="p-3 bg-gray-800 rounded-md text-white">Wrap Up</p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Right Panel */}
      <div className="col-span-1 space-y-8">
        {/* Daily Summary */}
        <Card className="bg-gray-800/50 border-gray-800">
          <CardHeader className="flex flex-row justify-between items-center pt-6 space-y-0">
            <h3 className="text-lg font-semibold text-white">Daily Summary</h3>
            <Button
              variant="ghost"
              size="icon"
              className="p-1 text-gray-400 hover:text-white hover:bg-gray-700"
              onClick={async () => {
                await syncCalendar();
              }}
            >
              <RefreshCw className="h-4 w-4" />
            </Button>
          </CardHeader>
          <CardContent className="pb-6">
            <p className="text-sm text-gray-400 leading-relaxed">
              Today was a productive day, focusing on key tasks and meetings. You successfully
              completed Project X and had effective communication with the team and clients. There was a
              minor deviation with the client call running over and a period of distraction in the
              afternoon. Overall, you maintained a good balance between planned and actual activities.
            </p>
          </CardContent>
        </Card>

        {/* Reality Check */}
        <Card className="bg-gray-800/50 border-gray-800">
          <CardHeader className="pt-6">
            <h3 className="text-lg font-semibold text-white">Reality Check</h3>
          </CardHeader>
          <CardContent className="pb-6">
            <div className="flex gap-4 justify-between items-center mb-2">
              <p className="text-sm font-medium text-gray-400">Reality Score</p>
              <p className="text-lg font-bold text-white">85%</p>
            </div>
            <div className="w-full bg-gray-700 rounded-full h-2.5 mb-4">
              <div className="bg-blue-600 h-2.5 rounded-full" style={{ width: '85%' }}></div>
            </div>
            <div className="space-y-3">
              <div className="flex items-start gap-3">
                <AlertTriangle className="h-5 w-5 text-yellow-400 mt-1" />
                <div>
                  <p className="font-medium text-white">Client Call ran 15 mins over</p>
                  <p className="text-sm text-gray-400">Planned: 11:00 AM, Actual: 11:00 AM - 11:15 AM</p>
                </div>
              </div>
              <div className="flex items-start gap-3">
                <XCircle className="h-5 w-5 text-red-500 mt-1" />
                <div>
                  <p className="font-medium text-white">Unplanned Social Media Break</p>
                  <p className="text-sm text-gray-400">Between 2:00 PM - 3:00 PM</p>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Suggestions for Tomorrow */}
        <Card className="bg-gray-800/50 border-gray-800">
          <CardHeader className="pt-6">
            <h3 className="text-lg font-semibold text-white">Suggestions for Tomorrow</h3>
          </CardHeader>
          <CardContent className="pb-6">
            <ul className="space-y-3">
              <li className="flex items-start gap-3">
                <CheckCircle className="h-5 w-5 text-green-500 mt-1" />
                <p className="text-sm text-gray-300">Set hard stops for meetings to stay on schedule.</p>
              </li>
              <li className="flex items-start gap-3">
                <CheckCircle className="h-5 w-5 text-green-500 mt-1" />
                <p className="text-sm text-gray-300">Use a website blocker during focused work blocks.</p>
              </li>
              <li className="flex items-start gap-3">
                <CheckCircle className="h-5 w-5 text-green-500 mt-1" />
                <p className="text-sm text-gray-300">Schedule short, intentional breaks to avoid burnout.</p>
              </li>
            </ul>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}