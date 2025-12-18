export const API_URL = () => {

  const apiUrl = process.env.NEXT_PUBLIC_API_URL ?? '';
  const env = process.env.NEXT_PUBLIC_ENV ?? '';

  if (env === "staging" || env === "development") {
    return `${apiUrl}/${env}`
  } else if (env === "production") {
    return apiUrl
  }
  return ''
}


export const fetcher = (url: string) => fetch(url).then(r => r.json())
