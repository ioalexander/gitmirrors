import axios from "axios";
import type { IApiResponse } from "~/types/api";
import type { DashboardData } from "~/types/dashboard";

export const aggregateApi = (config: { baseUrl: string }) => {
  const axiosBase = axios.create({
    baseURL: config.baseUrl + "/aggregate",
    withCredentials: true,
    validateStatus: (status) => status >= 200 && status < 399,
  });

  return {
    async getDashboardData(): Promise<
      IApiResponse<{ dashboard: DashboardData }>
    > {
      const { data: payload } = await axiosBase.get(`/dashboard`);

      return payload;
    },
  };
};
