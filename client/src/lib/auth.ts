import { createServerFn } from '@tanstack/react-start'
import { getRequest } from '@tanstack/react-start/server'

const API = import.meta.env.VITE_API_URL ?? 'http://localhost:8000'

export const getMe = createServerFn({ method: 'GET' }).handler(async () => {
  try {
    const cookie = getRequest().headers.get('cookie') ?? ''

    const res = await fetch(`${API}/api/v1/auth/me`, {
      headers: { cookie },
    })

    if (!res.ok) return null

    const contentType = res.headers.get('content-type')
    if (!contentType?.includes('application/json')) return null

    return res.json() as Promise<{ id: string; account: string }>
  } catch {
    return null
  }
})

export function loginWithGoogle() {
  window.location.href = `${API}/o2p/user/login`
}

export async function logout() {
  await fetch(`${API}/o2p/user/logout`, {
    method: 'POST',
    credentials: 'include',
  })
  window.location.reload()
}
