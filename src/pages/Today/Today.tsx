'use client';

import { Card, CardHeader, CardContent } from '@/components/ui/card';
import DailySummary  from '@/components/SummaryPanel/DailySummary';
import Suggestions from '@/components/SummaryPanel/Suggestions';
import RealityCheck  from '@/components/SummaryPanel/RealityCheck';

export default function Today() {

  return (
    <div className="grid grid-cols-3 p-8 gap-8">
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
        <DailySummary />

        {/* Reality Check */}
        <RealityCheck />

        {/* Suggestions for Tomorrow */}
        <Suggestions />
      </div>
    </div>
  );
}