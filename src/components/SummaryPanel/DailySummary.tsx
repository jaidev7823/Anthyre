import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { RefreshCw } from "lucide-react";
import { syncCalendar, daily_summary } from "@/lib/utils";
import { Card, CardHeader, CardContent } from "@/components/ui/card";

export const DailySummaryCard = () => {
  const [summary, setSummary] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  const fetchSummary = async () => {
    setLoading(true);
    try {
      const result = await daily_summary();
      setSummary(result as string);
    } catch (err: unknown) {
      console.error("Failed to fetch summary:", err);
      const errorMsg = err instanceof Error ? err.message : String(err);
      setSummary(`Failed to load summary: ${errorMsg}`);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchSummary();
  }, []);

  return (
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
          {loading ? "Loading daily summary..." : summary}
        </p>
      </CardContent>
    </Card>
  );
};

export default DailySummaryCard;