<template>
  <div class="wrapper" v-title data-title="归档">
    <i-header></i-header>
    <div>
      <h1>归档</h1>
      <i-archive></i-archive>
    </div>
    <i-footer></i-footer>
  </div>
</template>

<script>
import Header from '@/components/shared/_header'
import Footer from '@/components/shared/_footer'
import Archive from '@/components/shared/_archive'

export default {
  name: 'Home',
  components: {
    'i-header': Header,
    'i-footer': Footer,
    'i-archive': Archive
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="less" scoped>
@import url(../assets/_variables.less);
.wrapper {
  width: 100%;
  max-width: 900px;
  margin: 0 auto;
  padding: @large-margin;
  h1 {
    color: @color-accent;
  }
}
@media (max-width: @screen-size) {
  .wrapper {
    padding: @small-margin;
  }
}
</style>
