<template>
  <ul class="_menu">
    <li v-for="menu in site.menus" :key="menu.title">
      <router-link v-if="menu.type=='router-link'" :to="menu">{{menu.title}}</router-link>
      <a v-if="menu.type=='link'" :href="menu.path" :target="menu.target">{{menu.title}}</a>
    </li>
  </ul>
</template>
<script>
import { mapGetters } from 'vuex'
export default {
  data() {
    return {}
  },
  computed: {
    ...mapGetters(['site'])
  }
}
</script>
<style lang="less" scoped>
@import url(../../assets/_variables.less);
@import url(../../assets/_mixins.less);
._menu {
  letter-spacing: 0.01em;
  font-size: 0.8rem;
  font-style: normal;
  font-weight: 200;
  color: @color-accent;
  white-space: nowrap;
  display: inline-block;
  overflow: auto;
  padding: 0px;
  ul {
    list-style-type: none;
    margin: 0;
    padding: 0;
    line-height: 15px;
  }
  a {
    margin-right: 15px;
    color: @color-accent;
    text-decoration: none;
  }
  a:hover {
    .underline(5px, @color-accent);
  }
  li {
    display: inline-block;
    vertical-align: middle;
    margin-right: 15px;
    border-right: 1px dotted @color-accent;
  }
  li:last-child {
    border-right: 0;
    margin-right: 0;
  }
}
</style>
