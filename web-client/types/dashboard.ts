import type { Repository } from "./repository";

export type DashboardData = {
  totalRepositories: number;
  enabled: number;
  disabled: number;
  cloneFrequencyPastDay: number;
  cloneFrequencyPastWeek: number;
  logsFrequencyPastDay: number;
  logsFrequencyPastWeek: number;
  failedCloneJobFrequencyPastDay: number;
  failedCloneJobFrequencyPastWeek: number;
  lastClonedRepos: Repository[];
};
