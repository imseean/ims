<template>
    <div>
        <p>
            很早就想开始写博客，无奈一直纠结用什么程序，导致在写博客这件事情上一拖再拖。
            想了很久还是自己造个轮子吧，正好可以趁机练习下学习了很久还是不会的Rust ╮(╯_╰)╭。
        </p>
        <p>
            既然决定了造轮子，那么问题来了。静态还是动态？看了看我的乞丐版服务器...静态吧。
            考虑到自己写博客的目的“总结归纳”大于“分享“，就不考虑SEO了，SPA体验还好点。

        </p>
        <p>
            决定基本思路就是使用markdown保存内容，然后用Rust写个程序转化为json文件。
            前端使用VueJs，获取json文件并显示（基本上就是模仿大名鼎鼎的Hexo和Hugo）。最后这个半成品就诞生了！
        </p>
        <p>
            源代码：<a href="https://github.com/obaiyan/ims" target="_black">Github</a>
        </p>
    </div>
</template>
<script>
export default {}
</script>
<style lang="less" scoped>

</style>
