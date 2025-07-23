import axios from "axios";
import type { NuxtServerInitOptions } from "~/plugins/init.server";
import type { IApiResponse } from "~/types/api";

export const userApi = (
  config: { baseUrl: string },
  nuxtServerInitOptions?: NuxtServerInitOptions,
) => {
  const axiosBase = axios.create({
    baseURL: config.baseUrl + "/user",
    withCredentials: true,
    validateStatus: (status) => status >= 200 && status < 399,
  });

  return {
    async login(params: { username: string; password: string }): Promise<
      IApiResponse<{
        user: { id: string; createdAt: string; updatedAt: string };
      }>
    > {
      const { data: payload } = await axiosBase.post("/login", params);

      return payload;
    },
    async logout() {
      await axiosBase.post("/logout");
    },
    async getMe(nuxtServerInitOptions?: NuxtServerInitOptions): Promise<
      IApiResponse<{
        user: {
          id: string;
          username: string;
          createdAt: string;
          updatedAt: string;
        };
      }>
    > {
      const { data: payload } = await axiosBase.get("/me", {
        headers: {
          Cookie: nuxtServerInitOptions?.serverSideCookiesRaw,
        },
      });

      return payload;
    },
    async changePassword(params: { password: string }): Promise<
      IApiResponse<{
        user: { id: string; createdAt: string; updatedAt: string };
      }>
    > {
      const { data: payload } = await axiosBase.post(
        "/change-password",
        params,
      );

      return payload;
    },
  };
};
