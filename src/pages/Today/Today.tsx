"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import { Card, CardHeader, CardContent } from "@/components/ui/card";
import DailySummary from "@/components/SummaryPanel/DailySummary";
import Suggestions from "@/components/SummaryPanel/Suggestions";
import RealityCheck from "@/components/SummaryPanel/RealityCheck";

type Batch = {
  start_hour: number;
  end_hour: number;
  label: string;
  is_event: boolean;
};

export default function Today() {
  const [batches, setBatches] = useState<Batch[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    invoke<Batch[]>("fetch_batches")
      .then((res) => setBatches(res))
      .catch((err) => console.error("Failed to fetch batches:", err))
      .finally(() => setLoading(false));
  }, []);

  return (
    <div className="grid grid-cols-3 p-8 gap-8">
      {/* Main Body (Batches) */}
      <div className="col-span-2 space-y-4">
        {loading && (
          <p className="text-gray-400 text-center">Loading your schedule...</p>
        )}

        {!loading && batches.length === 0 && (
          <p className="text-gray-400 text-center">No data available</p>
        )}

        {batches.map((batch, index) => (
          <Card
            key={index}
            className={`border-gray-800 bg-slate-900 ${
              batch.is_event ? "ring-1 ring-green-600" : ""
            }`}
          >
            <CardHeader className="px-4 py-3 bg-gray-800/50 flex justify-between items-center">
              <h3 className="font-semibold text-white">
                {batch.label} ({batch.start_hour}:00 → {batch.end_hour}:59)
              </h3>
            </CardHeader>
            <CardContent className="p-4 grid grid-cols-2 gap-4">
              <div>
                <h4 className="font-medium text-gray-400 mb-2">Planned</h4>
                <div className="space-y-2">
                  <p className="p-3 bg-gray-800 rounded-md text-gray-500 italic">
                    Empty
                  </p>
                </div>
              </div>
              <div>
                <h4 className="font-medium text-gray-400 mb-2">Actual</h4>
                <div className="space-y-2">
                  <p className="p-3 bg-gray-800 rounded-md text-gray-500 italic">
                    Empty
                  </p>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Right Panel */}
      <div className="col-span-1 space-y-8">
        <DailySummary />
        <RealityCheck />
        <Suggestions />
      </div>
    </div>
  );
}
