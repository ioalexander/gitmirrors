import axios from "axios";
import type { NuxtServerInitOptions } from "~/plugins/init.server";
import type { IApiResponse } from "~/types/api";

export const authApi = (
  config: { baseUrl: string },
  nuxtServerInitOptions?: NuxtServerInitOptions,
) => {
  const axiosBase = axios.create({
    baseURL: config.baseUrl + "/auth",
    withCredentials: true,
    validateStatus: (status) => status >= 200 && status < 399,
  });

  return {
    async login(params: { username: string; password: string }): Promise<
      IApiResponse<{
        user: { id: string; createdAt: string; updatedAt: string };
        session: { id: string; createdAt: string; updatedAt: string };
      }>
    > {
      const { data: payload } = await axiosBase.post("/login", params);

      return payload;
    },
    async logout() {
      await axiosBase.post("/logout");
    },
    async getMe(): Promise<
      IApiResponse<{
        user: { id: string; createdAt: string; updatedAt: string };
        session: { id: string; createdAt: string; updatedAt: string };
      }>
    > {
      const { data: payload } = await axiosBase.get("/me", {
        headers: {
          Cookie: nuxtServerInitOptions?.serverSideCookiesRaw,
        },
      });

      return payload;
    },
  };
};
