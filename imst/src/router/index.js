import Home from '@/components/Home'
import Archive from '@/components/Archive'
import Tag from '@/components/Tag'
import Post from '@/components/Post'
// const Home = () => import('@/components/Home.vue')
// const Archive = () => import('@/components/Archive.vue')
// const Tag = () => import('@/components/Tag.vue')
// const Post = () => import('@/components/Post.vue')

export default {
  routes: [
    {
      path: '/',
      name: 'Home',
      component: Home
    },
    {
      path: '/archive',
      name: 'Archive',
      component: Archive
    },
    {
      path: '/tag/:name',
      component: Tag
    },
    {
      path: '/tag/',
      name: 'Tag',
      component: Tag
    },
    {
      path: '/post/:id',
      name: 'Post',
      component: Post
    }
  ]
}
