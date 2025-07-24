<template>
  <div :class="[$style.sidebar, { [$style.fold]: isSidebarFold }]">
    <div :class="$style.logo">
      <img
        src="/images/logo_150px.webp"
        alt="gitmirrors logo webp"
        width="150"
      />
      <button :class="$style.foldSidebar" @click="toggleSidebarFold">
        <Icon v-if="isSidebarFold" name="ci:hamburger-lg" />
        <Icon v-else name="material-symbols:arrow-back-ios" />
      </button>
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
        >
          <Icon :class="$style.icon" :name="url.icon" />
          <span :class="$style.title">
            {{ url.title }}
          </span>
          <div :class="$style.marker"></div
        ></NuxtLink>
      </div>
      <div :class="$style.bottomlinks">
        <NuxtLink to="/dashboard/account" :class="$style.account">
          <Icon :class="$style.icon" name="material-symbols:account-circle" />
          <span :class="$style.username">
            {{ user?.username }}
          </span>
        </NuxtLink>
      </div>
    </aside>
  </div>
</template>

<script setup lang="ts">
import { useUiStore } from "~/store/ui.store";
import { useUserStore } from "~/store/user.store";

const dashboardUrls = [
  {
    title: "Dashboard",
    url: "/dashboard",
    icon: "material-symbols:dashboard",
  },
  {
    title: "Repositories",
    url: "/dashboard/repository",
    icon: "mdi:source-repository",
  },
];

const route = useRoute();
const currentUrl = computed(() => route.path);

const userStore = useUserStore();
const uiStore = useUiStore();

const user = computed(() => userStore.user);
const isSidebarFold = computed(() => uiStore.isSidebarFold);

const toggleSidebarFold = () => {
  uiStore.SET_SIDEBAR_FOLD(!uiStore.isSidebarFold);
  console.log(uiStore.isSidebarFold);
};
</script>
<style lang="scss" module>
.sidebar {
  background: var(--black-transparent);
  backdrop-filter: var(--black-transparent-backdrop);
  width: 100%;
  border-right: 1px solid var(--gray-900);
  min-height: 100vh;
  filter: var(--shadow-large);
  transition: 0.5s;

  .logo {
    position: relative;
    width: 100%;
    height: 80px;
    border-bottom: 1px solid var(--gray-900);
    text-align: center;
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 20px;
    user-select: none;

    img {
      pointer-events: none;
      opacity: 1;
      transition: 0.2s;
    }

    .foldSidebar {
      position: absolute;
      font-size: 40px;
      width: 60px;
      height: 60px;
      left: 10px;
      top: 50%;
      transform: translate(0px, -50%);
      background: none;
      display: flex;
      justify-content: center;
      align-items: center;
      padding: 0;
      transition: 0.2s;
      border-radius: 8px;

      & > * {
        font-size: 20px;
      }

      &:hover {
        background: var(--black-transparent-hover);
      }
    }
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

        .title {
          font-size: 18px;
          transition: 0.5s;
        }

        .icon {
          font-size: 26px;
          margin-right: 16px;
          transition: 0.5s;
          margin-left: 5px;
        }

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
          background: var(--black-transparent-hover);
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
        align-items: center;

        .username {
          font-size: 16px;
        }

        .icon {
          font-size: 20px;
          margin-right: 8px;
        }

        &:hover {
          background: var(--black-transparent-hover);
        }
      }
    }
  }
}

.sidebar.fold {
  width: 80px !important;

  .logo img {
    opacity: 0;
  }

  .container {
    .linkslist {
      .link {
        .title {
          font-size: 0;
          opacity: 0;
        }
        .icon {
          margin-right: 0;
        }
      }
    }

    .bottomlinks {
      .account {
        width: 100%;
        margin: 0;
        border: none;
        border-radius: 0;
        justify-content: center;

        .username {
          font-size: 0px;
        }

        .icon {
          font-size: 20px;
          margin-right: 0px;
        }
      }
    }
  }
}
</style>
