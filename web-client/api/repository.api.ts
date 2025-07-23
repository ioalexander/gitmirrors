import axios from "axios";
import type { IApiResponse } from "~/types/api";
import type { Repository } from "~/types/repository";

export const repositoryApi = (config: { baseUrl: string }) => {
  const axiosBase = axios.create({
    baseURL: config.baseUrl + "/repository",
    withCredentials: true,
    validateStatus: (status) => status >= 200 && status < 399,
  });

  return {
    async getRepositories(): Promise<
      IApiResponse<{
        repositories: Repository[];
      }>
    > {
      const { data: payload } = await axiosBase.get("/");

      return payload;
    },
    async addRepository(
      data: Omit<Repository, "id" | "createdAt" | "updatedAt" | "lastCloneAt">,
    ): Promise<IApiResponse<{ createdRepository: Repository }>> {
      const { data: payload } = await axiosBase.post("/", data);

      return payload;
    },
  };
};
