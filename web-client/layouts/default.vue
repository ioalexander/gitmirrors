<template>
  <div :class="$style.container">
    <Sidebar />
    <div :class="$style.content">
      <slot />
    </div>
  </div>
</template>

<style lang="scss" module>
.container {
  width: 100%;
  display: grid;
  grid-template-columns: 300px 1fr;
  grid-gap: 0;

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
    }

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

        &:hover {
          background: var(--black-100);
        }
      }
    }
  }

  .content {
    width: 100%;
    min-height: 100vh;
    padding: 30px;
  }
}
</style>
