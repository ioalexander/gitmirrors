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
      <ControlsPanel :class="$style.logsChart">
        <Bar
          :data="logsBarChartData"
          :class="$style.chart"
          :options="{ responsive: true }"
        />
      </ControlsPanel>
    </transition-group>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, reactive } from "vue";
import { useToast } from "vue-toastification";
import { useUserStore } from "~/store/user.store";
import type { DashboardData, DailyLogCount } from "~/types/dashboard";
import { Doughnut, Bar } from "vue-chartjs";
import {
  Chart as ChartJS,
  ArcElement,
  Tooltip,
  Legend,
  CategoryScale,
  LinearScale,
  BarElement,
} from "chart.js";

ChartJS.register(
  ArcElement,
  Tooltip,
  Legend,
  CategoryScale,
  LinearScale,
  BarElement,
);

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
        backgroundColor: [accentColor.value, "#606060"],
        data: [state.dashboard?.enabled ?? 0, state.dashboard?.disabled ?? 0],
        borderWidth: 0,
      },
    ],
  };
});

const accentColor = computed(
  () =>
    getComputedStyle(document.documentElement)
      .getPropertyValue("--accent-color-blue")
      .trim() || "#2090AA",
);

const errorColor = computed(
  () =>
    getComputedStyle(document.documentElement)
      .getPropertyValue("--status-color-error")
      .trim() || "#AA0000",
);

// Prepare bar chart data for daily logs and errors
const logsBarChartData = computed(() => {
  if (!state.dashboard) return { labels: [], datasets: [] };

  const labels = state.dashboard.dailyLogs.map((d) => d.day);
  return {
    labels,
    datasets: [
      {
        label: "Logs",
        backgroundColor: accentColor?.value,
        data: state.dashboard.dailyLogs.map((d) => d.count),
      },
      {
        label: "Error Logs",
        backgroundColor: errorColor?.value,
        data: state.dashboard.dailyErrorLogs.map((d) => d.count),
      },
    ],
  };
});

const logsBarChartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    y: {
      beginAtZero: true,
      ticks: {
        stepSize: 1,
      },
    },
  },
  plugins: {
    legend: {
      position: "top",
    },
  },
};

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

  .list {
    display: grid;
    grid-template-columns: 450px 2fr;
    grid-gap: 20px;
  }
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
      margin-bottom: 10px;
    }
    .row {
      color: var(--gray-600);
      font-size: 14px;
      margin-bottom: 4px;
    }
  }
}

.logsChart {
  .chart {
    width: 100% !important;
  }
}

.list > * {
  opacity: 0;
  transform: translateY(10px);
  animation: fadeInUp 0.5s forwards;
}

$list-max: 100;
$list-delay-step: 0.05s;
$list-delay-start: 0s;

@for $i from 1 through $list-max {
  .list > *:nth-child(#{$i}) {
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

@media all and (max-width: 1300px) {
  .container {
    .list {
      grid-template-columns: 1fr;
    }
  }
}
</style>
