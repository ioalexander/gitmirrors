<template>
  <div :class="$style.container">
    <transition-group name="fade" tag="div" :class="$style.list">
      <slot />
    </transition-group>
  </div>
</template>
<style lang="scss" module>
.container {
  width: 100%;
  height: 100%;
}

.list {
  width: 100%;
  display: grid;
  grid-template-columns: 1fr 1fr 1fr 1fr;
  grid-gap: 30px;

  @media all and (max-width: 1800px) {
    & {
      grid-template-columns: 1fr 1fr 1fr;
    }
  }

  @media all and (max-width: 1600px) {
    & {
      grid-template-columns: 1fr 1fr;
    }
  }
  @media all and (max-width: 900px) {
    & {
      grid-template-columns: 1fr;
    }
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
</style>
