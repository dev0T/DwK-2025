export const API_URL = process.env.NEXT_PUBLIC_API_URL ?? ''

export const fetcher = (url: string) => fetch(url).then(r => r.json())
