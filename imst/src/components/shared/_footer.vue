<template>
  <footer class="footer">
    <div class="footer-left"> Copyright © 2018 BAIYAN</div>
    <div class="footer-right">
      <nav>
        <ul>
          <li v-for="menu in site.menus" :key="menu.title">
            <router-link v-if="menu.type=='router-link'" :to="menu">{{menu.title}}</router-link>
            <a v-if="menu.type=='link'" :href="menu.path" :target="menu.target">{{menu.title}}</a>
          </li>
        </ul>
      </nav>
    </div>
  </footer>
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
</script><style lang="less" scoped>
@import url(../../assets/_variables.less);
.footer {
  padding-top: 10px;
  padding-bottom: 10px;
  color: #666;
  font-size: 11px;
  width: 100%;
  margin: 0 auto 1rem auto;
  vertical-align: top;
  text-align: center;
  ul {
    margin: 0;
    padding: 0;
    list-style: none;
    li {
      display: inline-block;
      vertical-align: middle;
      margin-right: 15px;
      border-right: 1px solid #4a4a4a;
      a {
        margin-right: 15px;
      }
    }
    li:last-child {
      border-right: 0;
      margin-right: 0;
      a {
        margin-right: 0;
      }
    }
  }
  a {
    color: #666;
  }
  a:hover {
    color: #999;
  }
  .footer-left {
    height: 20px;
    line-height: 20px;
    vertical-align: middle;
  }
}

@media (min-width: @screen-size) {
  .footer {
    padding-top: 50px;
    padding-bottom: 20px;
    .footer-left {
      float: left;
    }
    .footer-right {
      float: right;
    }
  }
}
</style>
