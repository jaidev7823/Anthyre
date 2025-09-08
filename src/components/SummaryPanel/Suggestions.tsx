import { CheckCircle } from 'lucide-react';
import { Card, CardHeader, CardContent } from '@/components/ui/card';

export const SuggestionsCard = () => {
  return (
    <Card className="bg-gray-800/50 border-gray-800">
      <CardHeader className="pt-6">
        <h3 className="text-lg font-semibold text-white">
          Suggestions for Tomorrow
        </h3>
      </CardHeader>
      <CardContent className="pb-6">
        <ul className="space-y-3">
          <li className="flex items-start gap-3">
            <CheckCircle className="h-5 w-5 text-green-500 mt-1" />
            <p className="text-sm text-gray-300">
              Set hard stops for meetings to stay on schedule.
            </p>
          </li>
          <li className="flex items-start gap-3">
            <CheckCircle className="h-5 w-5 text-green-500 mt-1" />
            <p className="text-sm text-gray-300">
              Use a website blocker during focused work blocks.
            </p>
          </li>
          <li className="flex items-start gap-3">
            <CheckCircle className="h-5 w-5 text-green-500 mt-1" />
            <p className="text-sm text-gray-300">
              Schedule short, intentional breaks to avoid burnout.
            </p>
          </li>
        </ul>
      </CardContent>
    </Card>
  );
};

export default SuggestionsCard;