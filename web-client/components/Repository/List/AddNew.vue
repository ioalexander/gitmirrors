<template>
  <div :class="$style.item">
    <div :class="$style.border">
      <Icon
        :class="$style.plus"
        name="material-symbols:add-circle-outline-rounded"
      />
    </div>
  </div>
</template>
<style lang="scss" module>
.item {
  height: 200px;
  border-radius: 16px;
  cursor: pointer;

  .border {
    width: 100%;
    height: 100%;

    border: 2px dashed var(--gray-900);
    border-radius: 16px;
    padding: 30px;

    display: flex;
    justify-content: center;
    align-items: center;
    .plus {
      font-size: 40px;
      color: var(--gray-900);
    }
  }

  &:hover {
    background: var(--black-100);
  }
}
</style>
