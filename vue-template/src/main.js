// The Vue build version to load with the `import` command
// (runtime-only or standalone) has been set in webpack.base.conf with an alias.
import { Vue, Vuex, Router } from '@/init.js'
import App from './App'
import storeData from './store'
import routerData from './router'

var store = new Vuex.Store(storeData)
Vue.directive('title', {
  update: function(el, binding) {
    if (el.dataset.title != null && el.dataset.title !== '') {
      document.title = el.dataset.title + ' | ' + store.getters.site.title
    } else {
      document.title = store.getters.site.title
    }
  },
  inserted: function(el, binding) {
    if (el.dataset.title != null && el.dataset.title !== '') {
      document.title = el.dataset.title + ' | ' + store.getters.site.title
    } else {
      document.title = store.getters.site.title
    }
  }
})
store
  .dispatch('init')
  .then(results => {
    var mode = store.getters.site.mode
    routerData.mode = mode
    var router = new Router(routerData)
    router.beforeEach((to, from, next) => {
      next()
    })
    /* eslint-disable no-new */
    new Vue({
      el: '#app',
      router,
      store,
      components: { App },
      template: '<App/>'
    })
  })
  .catch(function(reason) {
    console.log(reason)
  })
