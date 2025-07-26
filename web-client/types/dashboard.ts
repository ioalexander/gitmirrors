import type { Repository } from "./repository";

export type DailyLogCount = {
  day: string; // YYYY-MM-DD
  count: number;
};

export type DashboardData = {
  totalRepositories: number;
  enabled: number;
  disabled: number;
  lastClonedRepos: Repository[];
  dailyLogs: DailyLogCount[];
  dailyErrorLogs: DailyLogCount[];
};
