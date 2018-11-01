<template>
  <div class="wrapper" v-title data-title="标签">
    <i-header></i-header>
    <div>
      <h1>标签</h1>
      <div class="tags">
        <a v-scroll-to="{
        'el':'#a'+ hash(tag.name)
      }" class="tag" v-for="tag in tags" :key="tag.name">{{tag.name}}({{tag.list.length}})</a>
      </div>
      <div  v-for="tag in tags" :key="tag.name">
        <h2 :id="'a'+hash(tag.name)">{{tag.name}}</h2>
        <ul class="posts">
          <li class="post" v-for="post in tag.list" :key="post.id">
            <div class="meta">
              <time :datetime="post.create_time">{{post.create_time}}</time>
            </div>
            <span>
              <router-link :to="{ name: 'Post', params: { id: post.id }}">{{post.title}}</router-link>
            </span>
          </li>
        </ul>
      </div>
    </div>
    <i-footer></i-footer>
  </div>
</template>

<script>
import { mapGetters } from 'vuex'
import Header from '@/components/shared/_header'
import Footer from '@/components/shared/_footer'

export default {
  name: 'Home',
  methods: {
    hash: function(value) {
      if (!value) {
        return 0
      }
      var hash = 0
      if (value.length === 0) return hash
      for (var i = 0; i < value.length; i++) {
        var chr = value.charCodeAt(i)
        hash = ((hash << 5) - hash) + chr
        hash |= 0
      }
      return hash
    }
  },
  mounted: function () {
    this.$nextTick(function () {
      let name = this.$route.params.name
      let element = document.querySelector('#a' + this.hash(name))
      this.$scrollTo(element)
    })
  },
  components: {
    'i-header': Header,
    'i-footer': Footer
  },
  computed: {
    ...mapGetters([
      'tags'
    ])
  },
  filters: {
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style lang="less" scoped>
@import url(../assets/_variables.less);

@media (min-width: @screen-size) {
  .wrapper {
    width: 100%;
    max-width: 900px;
    margin: 0 auto;
    padding: @large-margin;
    h1 {
      color: @color-accent;
    }
    .tags{
      .tag{
        display: inline-block;
        margin: 5px 10px;
      }
    }
    .posts {
      padding: 0;
      .post {
        list-style-type: none;
        margin-left: 0;
        margin-bottom: 1rem;
        .meta {
          display: inline-block;
          font-size: 14px;
          color: #666;
          min-width: 100px;
          margin-right: 16px;
        }
      }
    }
  }
}
@media (max-width: @screen-size) {
  .wrapper {
    padding: @small-margin;
    .posts .post {
      display: flex;
      margin-bottom: 5px;
      .meta {
        text-align: left;
      }
    }
  }
}
</style>
