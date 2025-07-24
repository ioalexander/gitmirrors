interface State {
  isSidebarFold: boolean;
}

export const useUiStore = defineStore("ui", {
  state: (): State => ({
    isSidebarFold: false,
  }),
  actions: {
    SET_SIDEBAR_FOLD(value: boolean) {
      this.isSidebarFold = value;
    },
  },
});
