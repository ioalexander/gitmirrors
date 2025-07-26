<template>
  <Topbar>
    <div :class="$style.topbarContent">
      <h1 :class="$style.title">Dashboard</h1>
    </div>
  </Topbar>
  <div :class="$style.container">
    <transition-group name="fade" tag="div" :class="$style.list">
      <ControlsPanel :class="$style.repositoriesDoughnut">
        <div :class="$style.text">
          <h2 :class="$style.title">Hello, {{ user?.username }}</h2>
          <p :class="$style.row">
            Total repositories: {{ state.dashboard?.totalRepositories }}
          </p>
          <p :class="$style.row">
            Enabled/Disabled: {{ state.dashboard?.enabled }} /
            {{ state.dashboard?.disabled }}
          </p>
        </div>
        <Doughnut :data="repositoriesDoughnutData" :class="$style.doughnut" />
      </ControlsPanel>
    </transition-group>
  </div>
</template>
<script setup lang="ts">
import { useToast } from "vue-toastification";
import { useUserStore } from "~/store/user.store";
import type { DashboardData } from "~/types/dashboard";
import { Doughnut } from "vue-chartjs";
import { Chart as ChartJS, ArcElement, Tooltip, Legend } from "chart.js";
ChartJS.register(ArcElement, Tooltip, Legend);

const userStore = useUserStore();
const user = computed(() => userStore.user);
const api = useApi();
const toast = useToast();

const state = reactive<{ dashboard: DashboardData | undefined }>({
  dashboard: undefined,
});

const repositoriesDoughnutData = computed(() => {
  return {
    datasets: [
      {
        backgroundColor: ["#2090AA", "#202020"],
        data: [
          state.dashboard?.enabled as number,
          state.dashboard?.disabled as number,
        ],
        borderWidth: 0,
      },
    ],
  };
});

onMounted(async () => {
  try {
    const res = await api.aggregate.getDashboardData();
    state.dashboard = res.data.dashboard;
  } catch (e) {
    toast.error("Failed to fetch dashboard data.");
    console.error(e);
  }
});
</script>
<style lang="scss" module>
.container {
  width: 100%;
  height: 100%;
  display: grid;
  grid-template-columns: 1fr 2fr;
}

.topbarContent {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  .title {
    font-size: 20px;
  }
  .right {
    display: flex;
  }
}

.repositoriesDoughnut {
  display: grid;
  grid-template-columns: 1fr 200px;
  grid-gap: 20px;

  .doughnut {
    height: 200px !important;
    width: 200px !important;
  }

  .text {
    .title {
      font-size: 20px;
    }
    .row {
      color: var(--gray-600);
      font-size: 14px;
      margin-bottom: 4px;
    }
  }
}

.container > * {
  opacity: 0;
  transform: translateY(10px);
  animation: fadeInUp 0.5s forwards;
}

$list-max: 100;
$list-delay-step: 0.05s;
$list-delay-start: 0s;

@for $i from 1 through $list-max {
  .container > *:nth-child(#{$i}) {
    animation-delay: calc(
      #{$list-delay-start} + (#{$i} - 1) * #{$list-delay-step}
    );
  }
}

@keyframes fadeInUp {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
