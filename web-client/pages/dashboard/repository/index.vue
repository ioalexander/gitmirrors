<template>
  <div :class="$style.container">
    <div :class="$style.list">
      <div :class="$style.item">
        <div :class="$style.info">
          <h2 :class="$style.title">Repository</h2>
          <a :class="$style.url">https://github.com/test/test</a>
        </div>
        <div :class="$style.actions">
          <Button is-gray> View </Button>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { useUserStore } from "~/store/user.store";
import Button from "~/components/Controls/Button.vue";

const userStore = useUserStore();
const user = computed(() => userStore.user);
</script>
<style lang="scss" module>
.container {
  width: 100%;
  height: 100%;

  .list {
    width: 100%;
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    grid-gap: 30px;

    .item {
      padding: 30px;
      border: 1px solid var(--gray-900);
      border-radius: 16px;

      .info {
        margin-bottom: 30px;

        .title {
          font-size: 20px;
          margin-bottom: 4px;
        }

        .url {
          font-size: 14px;
        }
      }
    }
  }
}
</style>
