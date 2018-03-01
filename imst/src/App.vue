<template>
  <div id="app">
    <transition name="slide-fade">
      <router-view/>
    </transition>
  </div>
</template>

<script>
export default {
  name: 'App',
  components: {}
}
</script>

<style lang="less">
@import url(./assets/style.less);
#app {
  width: 100%;
  min-height: 100%;
  position: relative;
}
.slide-fade-enter-active {
  transition: all 1s ease;
}
.slide-fade-enter {
  opacity: 0;
}
.slide-fade-leave-active {
  // transition: all 1s ease;
}
.slide-fade-leave-to {
  opacity: 0;
}
</style>
