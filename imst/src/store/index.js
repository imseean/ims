import { Vue } from '@/init.js'

export default {
  state: {
    title: '',
    site: {
      menus: [],
      title: '',
      mode: 'hash'
    },
    posts: []
  },
  getters: {
    site: state => {
      return state.site
    },
    posts: state => {
      var options = {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      }
      return state.posts.map(post => {
        var data = { ...post }
        data.create_time = data.create_time.toLocaleDateString('en-US', options)
        return data
      })
    }
  },
  mutations: {
    setTitle(state, payload) {
      state.title = payload.title
    },
    setSite(state, payload) {
      state.site = payload.site
    },
    setPosts(state, payload) {
      state.posts = payload.posts
    }
  },
  actions: {
    loadSite({ commit }) {
      return new Promise((resolve, reject) => {
        Vue.http.get('site.json').then(
          response => {
            commit({
              type: 'setSite',
              site: response.body
            })
            resolve(response.body)
          },
          response => {
            reject(new Error(response.statusText))
          }
        )
      })
    },
    loadPosts({ commit }) {
      return new Promise((resolve, reject) => {
        Vue.http.get('post.json').then(
          response => {
            var posts = response.body.map(post => {
              post.create_time = new Date(post.create_time)
              return post
            })
            commit({
              type: 'setPosts',
              posts: posts
            })
            resolve(posts)
          },
          response => {
            reject(new Error(response.statusText))
          }
        )
      })
    },
    init({ dispatch }) {
      return Promise.all([dispatch('loadSite'), dispatch('loadPosts')])
    }
  }
}
