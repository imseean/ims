<template>
  <div id="container" class="wrapper">
  </div>
</template>
<script>
import 'gitalk/dist/gitalk.css'
import Gitalk from 'gitalk'
export default {
  mounted() {
    var id = this.id
    var gitalk = new Gitalk({
      clientID: 'd6bede35b03c7fe342cd',
      clientSecret: '27886c787d06b73b751f4a71cb08ca132938742a',
      repo: 'baiyan',
      owner: 'obaiyan',
      admin: ['obaiyan'],
      id: id,
      distractionFreeMode: false,
      language: 'zh-CN'
    })
    gitalk.render('container')
  },
  props: {
    id: String
  }
}
</script>
<style lang="less">

</style>
