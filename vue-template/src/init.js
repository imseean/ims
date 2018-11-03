import Vue from 'vue'
import VueScrollTo from 'vue-scrollto'
import VueResource from 'vue-resource'
import Vuex from 'vuex'
import Router from 'vue-router'

Vue.use(VueScrollTo)
Vue.use(VueResource)
Vue.use(Vuex)
Vue.use(Router)

Vue.config.productionTip = false
Vue.http.options.root = '/data'

export { Vue, VueResource, Vuex, Router, VueScrollTo }
