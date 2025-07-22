<template>
  <div :class="$style.sidebar">
    <div :class="$style.logo">
      <img
        src="/images/logo_150px.webp"
        alt="gitmirrors logo webp"
        width="150"
      />
    </div>
    <aside :class="$style.container">
      <div :class="$style.linkslist">
        <NuxtLink
          v-for="url in dashboardUrls"
          :class="[
            {
              [$style.link]: true,
              [$style.onThisPage]: currentUrl === url.url,
            },
          ]"
          :to="url.url"
          >{{ url.title }}
          <div :class="$style.marker"></div
        ></NuxtLink>
      </div>
      <div :class="$style.bottomlinks">
        <NuxtLink to="/dashboard/account" :class="$style.account">
          {{ user?.username }}
        </NuxtLink>
      </div>
    </aside>
  </div>
</template>

<script setup lang="ts">
import { useUserStore } from "~/store/user.store";

const dashboardUrls = [
  {
    title: "Dashboard",
    url: "/dashboard",
  },
  {
    title: "Repositories",
    url: "/dashboard/repository",
  },
];

const route = useRoute();
const currentUrl = computed(() => route.path);

const userStore = useUserStore();
const user = computed(() => userStore.user);
</script>
<style lang="scss" module>
.sidebar {
  width: 100%;
  border-right: 1px solid var(--gray-900);
  min-height: 100vh;

  .logo {
    width: 100%;
    height: 80px;
    border-bottom: 1px solid var(--gray-900);
    text-align: center;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 20px;
    pointer-events: none;
    user-select: none;
  }

  .container {
    width: 100%;
    height: calc(100% - 80px - 1px);
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    .linkslist {
      width: 100%;
      display: flex;
      flex-direction: column;

      .link {
        width: 100%;
        display: flex;
        color: var(--white);
        text-decoration: none;
        padding: 0px 20px;
        height: 80px;
        align-items: center;
        text-align: left;
        position: relative;

        .marker {
          position: absolute;
          right: 0;
          top: 50%;
          transform: translateY(-50%);
          width: 6px;
          height: 30px;
          background: var(--white);
          border-top-left-radius: 6px;
          border-bottom-left-radius: 6px;
          pointer-events: none;
          opacity: 0;
          transition: 0.2s;
        }

        &:hover {
          background: var(--black-100);
        }
      }

      .link.onThisPage {
        .marker {
          opacity: 1;
          transition: 0.2s;
        }
      }
    }

    .bottomlinks {
      width: 100%;
      padding-bottom: 30px;

      .account {
        width: calc(100% - 60px);
        display: flex;
        padding: 16px;
        border: 1px solid var(--gray-900);
        margin: 0 30px;
        border-radius: 8px;
        text-decoration: none;
        color: var(--white);

        &:hover {
          background: var(--black-100);
        }
      }
    }
  }
}
</style>
