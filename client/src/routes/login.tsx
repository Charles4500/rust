import { loginWithGoogle } from '@/lib/auth'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/login')({
  head: () => ({
    meta: [
      { title: 'Log in — Sentify' },
      { name: 'description', content: 'Log in to your Sentify account.' },
    ],
  }),
  component: Login,
})

function Login() {
  return (
    <div className="flex min-h-screen flex-col">
      <main className="flex flex-1 items-center justify-center px-6 py-20">
        <div className="w-full max-w-sm">
          <button
            onClick={loginWithGoogle}
            type="submit"
            className="inline-flex h-11 w-full items-center justify-center rounded-md bg-primary text-sm font-medium text-primary-foreground transition-opacity hover:opacity-90 cursor-pointer"
          >
            Continue to sign in →
          </button>
        </div>
      </main>
    </div>
  )
}
