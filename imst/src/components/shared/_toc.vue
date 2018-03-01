<template>
  <ol>
    <li v-for="item in list" v-bind:key="item.title">
      <a class="title" v-scroll-to="{
        'el':'#'+item.code
      }" :style="{opacity:opacity}" :class="{major:major}" v-html="item.title"></a>
      <toc :list="item.child" :level="level+1" v-if="item.child"></toc>
    </li>
  </ol>
</template>
<script>
export default {
  name: 'toc',
  props: {
    list: Array,
    level: Number
  },
  data() {
    var opacity = 1
    for (var i = 0; i < this.level; i++) {
      opacity -= 0.1
    }
    return {
      opacity: opacity,
      major: this.level === 1
    }
  }
}
</script>
<style lang="less" scoped>
@import url(../../assets/_variables.less);

ol {
  text-align: right;
  overflow: auto;
  list-style-type: none;
  font-size: 0.9em;
  line-height: 2em;
  padding: 0px;
  li {
    .major {
      &::before {
        content: '# ';
        color: @color-accent;
      }
    }
    .title {
      background: none;
      text-decoration: none;
      color: @color-normal;

      &:hover {
        color: @color-anti;
      }
    }
  }
}
@media (max-width: @screen-size) {
  ol {
    text-align: left;
    padding-left: 1em;
  }
}
</style>
