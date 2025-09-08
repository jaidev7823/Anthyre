import { AlertTriangle, XCircle } from 'lucide-react';
import { Card, CardHeader, CardContent } from '@/components/ui/card';

export const RealityCheckCard = () => {
  return (
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
          <div
            className="bg-blue-600 h-2.5 rounded-full"
            style={{ width: "85%" }}
          ></div>
        </div>
        <div className="space-y-3">
          <div className="flex items-start gap-3">
            <AlertTriangle className="h-5 w-5 text-yellow-400 mt-1" />
            <div>
              <p className="font-medium text-white">Client Call ran 15 mins over</p>
              <p className="text-sm text-gray-400">
                Planned: 11:00 AM, Actual: 11:00 AM - 11:15 AM
              </p>
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
  );
};

export default RealityCheckCard;