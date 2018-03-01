import Home from '@/components/Home'
import Archive from '@/components/Archive'
import Post from '@/components/Post'

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
      path: '/post/:id',
      name: 'Post',
      component: Post
    }
  ]
}
