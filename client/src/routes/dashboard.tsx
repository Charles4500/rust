import { getMe } from '@/lib/auth'
import { createFileRoute, redirect } from '@tanstack/react-router'

export const Route = createFileRoute('/dashboard')({
  beforeLoad: async () => {
    const user = await getMe()
    if (!user) throw redirect({ to: '/login' })
    return { user }
  },
  component: RouteComponent,
})

function RouteComponent() {
  const { user } = Route.useRouteContext()
  console.log('This is the current user', user)
  return <div>Hello, {user.account}!</div>
}
