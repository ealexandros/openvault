"use client";

import { Button } from "@/components/ui/shadcn/button";
import { Input } from "@/components/ui/shadcn/input";
import { cn } from "@/utils/cn";
import { ActivityIcon, SearchIcon, ShieldAlertIcon } from "lucide-react";
import { useState } from "react";

interface LogEntry {
  id: string;
  action: string;
  timestamp: string;
  type: "info" | "success" | "warning" | "error";
  details?: string;
}

const MOCK_LOGS: LogEntry[] = [
  { id: "1", action: "Vault Unlocked", timestamp: "2026-01-22 16:30", type: "success" },
  {
    id: "2",
    action: "File Added",
    timestamp: "2026-01-22 16:35",
    type: "info",
    details: "report_v2.pdf",
  },
  {
    id: "3",
    action: "Security Alert",
    timestamp: "2026-01-22 16:40",
    type: "warning",
    details: "Multiple failed password attempts",
  },
  {
    id: "4",
    action: "Settings Changed",
    timestamp: "2026-01-22 16:45",
    type: "info",
    details: "Decoy vault path updated",
  },
  { id: "5", action: "Vault Locked", timestamp: "2026-01-22 16:50", type: "info" },
];

const LogItem = ({ log }: { log: LogEntry }) => (
  <div className="group flex items-center justify-between rounded-xl border border-border/50 bg-muted/20 p-4 transition-all hover:border-border">
    <div className="flex items-center gap-4">
      <div
        className={cn(
          "flex items-center justify-center rounded-lg border p-2",
          log.type === "success"
            ? "border-emerald-500/20 bg-emerald-500/10 text-emerald-500"
            : log.type === "warning"
              ? "border-amber-500/20 bg-amber-500/10 text-amber-500"
              : log.type === "error"
                ? "border-red-500/20 bg-red-500/10 text-red-500"
                : "border-primary/20 bg-primary/10 text-primary",
        )}>
        {log.type === "warning" || log.type === "error" ? (
          <ShieldAlertIcon className="size-4" />
        ) : (
          <ActivityIcon className="size-4" />
        )}
      </div>
      <div>
        <p className="text-sm font-medium">{log.action}</p>
        {log.details && (
          <p className="mt-0.5 text-[10px] text-muted-foreground">{log.details}</p>
        )}
      </div>
    </div>
    <div className="text-right">
      <p className="font-mono text-[10px] text-muted-foreground">{log.timestamp}</p>
    </div>
  </div>
);

const LogsPage = () => {
  const [logFilter, setLogFilter] = useState<LogEntry["type"] | "all">("all");
  const [logSearch, setLogSearch] = useState("");

  const filteredLogs = MOCK_LOGS.filter(
    log => logFilter === "all" || log.type === logFilter,
  ).filter(
    log =>
      log.action.toLowerCase().includes(logSearch.toLowerCase()) ||
      log.details?.toLowerCase().includes(logSearch.toLowerCase()),
  );

  return (
    <div className="mx-auto max-w-5xl space-y-8">
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div>
            <h3 className="text-lg font-medium">Activity Logs</h3>
            <p className="text-xs text-muted-foreground">
              Historical record of all vault operations
            </p>
          </div>
          <Button
            variant="outline"
            size="sm"
            className="h-8 border-border/50 text-xs font-medium hover:bg-muted/50">
            Clear History
          </Button>
        </div>

        <div className="flex flex-col items-start justify-between gap-4 pb-2 sm:flex-row sm:items-center">
          <div className="flex items-center gap-1.5 rounded-xl border border-border/50 bg-muted/20 p-1">
            {["all", "info", "success", "warning", "error"].map(type => (
              <button
                key={type}
                onClick={() => setLogFilter(type as any)}
                className={cn(
                  "rounded-lg px-3 py-1.5 text-[10px] font-bold tracking-wider uppercase transition-all",
                  logFilter === type
                    ? "bg-primary text-primary-foreground shadow-sm"
                    : "text-muted-foreground hover:bg-muted/50 hover:text-foreground",
                )}>
                {type}
              </button>
            ))}
          </div>

          <div className="group/log-search relative w-full sm:w-64">
            <SearchIcon className="absolute top-1/2 left-3 size-3.5 -translate-y-1/2 text-muted-foreground transition-colors group-focus-within/log-search:text-primary" />
            <Input
              placeholder="Search log activity..."
              value={logSearch}
              onChange={e => setLogSearch(e.target.value)}
              className="h-9 rounded-xl border-border/50 bg-muted/20 pl-9 text-xs focus:ring-primary/20"
            />
          </div>
        </div>

        <div className="space-y-3">
          {filteredLogs.map(log => (
            <LogItem key={log.id} log={log} />
          ))}
        </div>
      </div>
    </div>
  );
};

export default LogsPage;
