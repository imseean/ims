<template>
  <header class="header" :class="{'with-menu':showMenu}">
    <a class="one" href="">
      <div class="logo" :style="{'background-image':'url('+logo+')'}"></div>
      <div class="title">
        <h1>{{site.title}}</h1>
      </div>
    </a>
    <div class="toggle" v-on:click="toggleMenu">
      <icon name="bars" scale="2"></icon>
    </div>
    <ul class="menu">
      <li v-for="menu in site.menus" :key="menu.title">
        <router-link v-if="menu.type=='router-link'" :to="menu">{{menu.title}}</router-link>
        <a v-if="menu.type=='link'" :href="menu.path" :target="menu.target">{{menu.title}}</a>
      </li>
    </ul>
  </header>
</template>
<script>
import Logo from '@/assets/logo.png'
import { mapGetters } from 'vuex'
export default {
  data() {
    return {
      logo: Logo,
      showMenu: false
    }
  },
  methods: {
    toggleMenu: function() {
      this.showMenu = !this.showMenu
    }
  },
  computed: {
    ...mapGetters(['site'])
  }
}
</script>
<style lang="less" scoped>
@import url(../../assets/_variables.less);
@import url(../../assets/_mixins.less);
header {
  width: 100%;
  .title {
    display: inline-block;
    h1 {
      letter-spacing: 0.01em;
      font-size: 1.5rem;
      line-height: 2rem;
      font-style: normal;
      font-weight: 700;
      color: @color-normal;
      margin-top: 0;
      margin-bottom: 0;
    }
  }
  .logo {
    float: left;
    margin-right: 20px;
    width: 50px;
    height: 50px;
    background-repeat: no-repeat;
    background-size: 50px 50px;
    border-radius: 5px;
    -webkit-filter: grayscale(100%);
    filter: grayscale(100%);
  }
  .toggle {
    display: none;
  }
  .menu {
    letter-spacing: 0.01em;
    font-size: 0.8rem;
    font-style: normal;
    font-weight: 200;
    color: @color-accent;
    ul {
      list-style-type: none;
      margin: 0;
      padding: 0;
      line-height: 15px;
    }
    a {
      margin-right: 15px;
      color: color-accent;
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
    .icon {
      display: none;
    }
    li:last-child {
      border-right: 0;
      margin-right: 0;
    }
  }

  a {
    color: inherit;
    text-decoration: none;
    background: none;
  }
  &:hover {
    .logo {
      -webkit-filter: none;
      filter: none;
    }
  }
}

@media (max-width: @screen-size) {
  header {
    .title {
      padding-top: 9px;
      padding-bottom: 9px;
    }
    .toggle {
      display: block;
      float: right;
      color: @color-accent;
      height: 50px;
      line-height: 50px;
      font-size: 1.5rem;
      margin-right: 10px;
      & > * {
        vertical-align: middle;
      }
    }
    ul.menu {
      display: none;
      padding: 0px;
      a:hover {
        background: none;
      }
      li {
        display: block;
        height: 2rem;
        padding-top: 1rem;
        padding-left: 70px;
        font-size: 1rem;
        display: block;
        border-right: 0;
      }
      li.icon {
        display: inline-block;
        position: absolute;
        top: 55px;
        right: 1rem;
      }
    }
    &.with-menu {
      ul.menu {
        display: block;
      }
    }
  }
}
</style>
