<template>
  <div class="wrapper" :class="{'with-toc':showTOC,'with-side':showSide}" v-title :data-title="post.title">
    <div class="side">
      <span class="toggle" v-on:click="toggleTOC">
        <icon name="bars" scale="1.5"></icon>
      </span>
      <div class="menu">
        <i-menu></i-menu>
      </div>
      <div class="toc">
        <i-toc :list="toc" :level="1"></i-toc>
      </div>
    </div>
    <div class="content-wrapper">
      <article class="post">
        <header class="header">
          <h1 class="title"> {{post.title}} </h1>
          <div class="meta">
            <!-- <span class="author">
              <span itemprop="name">Sean.W</span>
            </span> -->
            <div class="date">
              <time :datetime="post.create_time">{{post.create_time}}</time>
            </div>
            <div class="tags" v-if="post.tags.length>0">
              <icon name="tag" class="icon"></icon>
              <router-link class="tag" v-for="tag in post.tags" :key="tag" :to="{
                path: '/tag/' + tag,
              }">{{tag}}</router-link>
            </div>
          </div>
        </header>
        <div class="content" v-html="markdownContent"></div>
      </article>
      <i-footer></i-footer>
    </div>
  </div>
</template>
<script>
import marked from 'marked'
import TOC from '@/components/shared/_toc'
import Menu from '@/components/shared/_menu'
import Footer from '@/components/shared/_footer'
import 'vue-awesome/icons/bars'
import 'vue-awesome/icons/tag'
import Icon from 'vue-awesome/components/Icon'

export default {
  created() {
    this.loadPost()
  },
  data() {
    return {
      post: {
        title: '',
        tags: []
      },
      toc: [],
      showTOC: false,
      showSide: true
    }
  },
  methods: {
    loadPost: function() {
      this.$http.get('post/' + this.$route.params.id + '.json').then(
        response => {
          // get body data
          var options = {
            year: 'numeric',
            month: 'short',
            day: 'numeric'
          }
          response.body.create_time = new Date(
            response.body.create_time
          ).toLocaleDateString('en-US', options)
          this.post = response.body
        },
        response => {
          // error callback
        }
      )
    },
    toggleTOC: function() {
      this.showTOC = !this.showTOC
    },
    handleScroll: function(e) {
      if (document.documentElement.scrollTop > 0) {
        if (this.scrollTop < document.documentElement.scrollTop) {
          if (!this.showTOC) {
            this.showSide = false
          }
        } else {
          this.showSide = true
        }
      }
      this.scrollTop = document.documentElement.scrollTop
    }
  },
  computed: {
    markdownContent: function() {
      function hash(str) {
        var hash = 0
        if (str.length === 0) return hash
        for (var i = 0; i < str.length; i++) {
          var chr = str.charCodeAt(i)
          hash = ((hash << 5) - hash) + chr
          hash |= 0
        }
        return hash
      }

      function getParentItem(list, level) {
        var currentList = list
        var currentItem = null
        for (var i = 1; i <= level; i++) {
          if (currentList.length > 0) {
            currentItem = currentList[currentList.length - 1]
          } else {
            currentItem = {
              title: '********',
              level: i,
              code: 'a' + hash('********'),
              child: []
            }
            currentList.push(currentItem)
          }
          currentList = currentItem.child
        }
        return currentItem
      }
      var self = this
      var renderer = new marked.Renderer()
      renderer.heading = function(text, level) {
        var title = text.replace(/<[^>]+>/g, '')
        var item = {
          title: title,
          level: level,
          code: 'a' + hash(title),
          child: []
        }
        if (level === 1) {
          self.toc.push(item)
        } else {
          var parent = getParentItem(self.toc, level - 1)
          parent.child.push(item)
        }
        return (
          '<h' +
          level +
          ' id="' +
          item.code +
          '">' + text +
          '</h' +
          level +
          '>'
        )
      }
      if (this.post.content) {
        return marked(this.post.content, {
          renderer: renderer
        })
      }
    }
  },
  components: {
    'i-toc': TOC,
    'i-menu': Menu,
    'i-footer': Footer,
    'icon': Icon
  },
  mounted() {
    this.scrollTop = 0
    addEventListener('scroll', this.handleScroll)
  },
  destroyed() {
    removeEventListener('scroll', this.handleScroll)
  }
}
</script>
<style lang="less" scoped>
@import url(../assets/_variables.less);
@import url(../assets/_mixins.less);
.wrapper {
  overflow-x: hidden;
  .toggle {
    vertical-align: middle;
  }
  .menu {
    display: inline-block;
    vertical-align: middle;
    text-align: right;
  }
  .toc {
    overflow-y: auto;
  }

  .post {
    transition: margin-right 0.3s ease;
    .header,
    .content,
    .footer {
      margin: 0 auto;
    }
    .header {
      .title {
        margin-top: 0;
        margin-bottom: 0;
        text-transform: none;
        font-size: 1.5em;
        line-height: 1.25;
        color: @color-accent;
      }
      .meta {
        margin-top: 0;
        margin-bottom: 1rem;
        * {
          color: @color-normal-x;
          font-size: 0.85rem;
        }
      }
      .author {
        text-transform: uppercase;
        letter-spacing: 0.01em;
        font-weight: 700;
      }
      .date {
        display: inline;
      }
      .tags {
        display: inline;
        .icon {
          vertical-align: middle;
        }
        .tag {
          vertical-align: middle;
          &::after {
            content: ', ';
          }
          &:last-child {
            &::after {
              content: '';
            }
          }
        }
      }
    }
    .content /deep/ a {
      color: @color-normal;
      .underline(5px, @color-normal);
      &:hover {
        color: @color-anti;
        .underline(5px, @color-anti);
      }
    }
  }
}
@media (min-width: @screen-size) {
  .wrapper {
    padding: @large-margin;
    .toggle {
      position: fixed;
      top: 32px;
      right: 32px;
    }
    .toc {
      position: fixed;
      width: 400px;
      margin-top: 64px;
      padding-right: 32px;
      padding-left: 32px;
      margin-bottom: 32px;
      top: 0px;
      bottom: 0px;
      right: -400px;
      transition: right 0.3s ease;
    }
    .menu {
      position: fixed;
      top: -64px;
      right: 0px;
      margin-right: 64px;
      width: 400px;
      transition: top 0.3s ease;
    }
    .content-wrapper{
      max-width: 900px;
      margin: 0 auto;
      padding: 32px;
    }
    .post {
      .tags {
        display: inline-block;
        &::before {
          content: '|';
          margin-right: 10px;
        }
      }
    }
    &.with-toc {
      .toc {
        right: 0px;
        background: @color-main;
      }
      .menu {
        top: 32px;
      }
      transition: margin 0.3s ease;
      margin-right: 400px;
    }
  }
}
@media (max-width: @screen-size) {
  .wrapper {
    padding: @small-margin;
    .side {
      position: fixed;
      top: -64px;
      left: 0px;
      right: 0px;
      height: 64px;
      line-height: 64px;
      background: @color-main;
      transition: top 0.3s ease;
      z-index: 999;
    }
    .toggle {
      display: inline-block;
      vertical-align: middle;
      margin: 0px 16px;
    }
    .menu {
      display: inline-block;
      vertical-align: middle;
      padding: 0px;
      overflow-x: auto;
      position: absolute;
      margin: 0px;
      top: auto;
      right: 16px;
      left: 64px;
    }
    .toc {
      position: fixed;
      top: 64px;
      left: -60%;
      bottom: 16px;
      width: 60%;
      transition: left 0.3s ease;
    }
    .post {
      margin-top: 64px;
      position: relative;
      transition: left 0.3s ease;
      left: 0px;
      .tags {
        display: block;
        &::before {
          content: '';
        }
      }
      .header,
      .content,
      .footer {
        width: 100%;
      }
    }
    &.with-side {
      .side {
        top: 0px;
      }
    }
    &.with-toc {
      .toc {
        left: 0px;
      }
      .post {
        left: 70%;
      }
    }
  }
}
</style>
