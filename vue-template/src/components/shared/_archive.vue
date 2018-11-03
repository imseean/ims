<template>
  <ul class="posts">
    <li class="post" v-for="post in visiblePosts" :key="post.id">
      <div class="meta">
        <time :datetime="post.create_time">{{post.create_time}}</time>
      </div>
      <span>
        <router-link :to="{ name: 'Post', params: { id: post.id }}">{{post.title}}</router-link>
      </span>
    </li>
  </ul>
</template>
<script>
import { mapGetters } from 'vuex'
export default {
  props: {
    visibleCount: {
      type: Number,
      default: -1
    }
  },
  data() {
    return {
    }
  },
  computed: {
    visiblePosts: function() {
      if (this.visibleCount <= 0) {
        return this.posts
      } else {
        return this.posts.slice(0, this.visibleCount)
      }
    },
    ...mapGetters([
      'posts'
    ])
  }
}
</script>
<style lang="less" scoped>
@import url(../../assets/_variables.less);
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
@media (min-width: @screen-size) {
  .posts .post {
    display: flex;
    margin-bottom: 5px;
    .meta {
      text-align: left;
    }
  }
}
</style>
