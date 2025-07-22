<template>
  <div :class="$style.container">
    <h2 :class="$style.title">Welcome,</h2>
    <p :class="$style.paragraph">This is your account settings page</p>
    <Spacer :height="10" />
    <h2 :class="$style.title">Security</h2>
    <AccountChangePasswordForm />
  </div>
</template>
<script setup lang="ts">
import Spacer from "~/components/Controls/Spacer.vue";
</script>
<style lang="scss" module>
.container {
  width: 100%;

  .title {
    margin-bottom: 10px;
  }

  .paragraph {
    margin-bottom: 10px;
  }
}
</style>
