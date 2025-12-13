import type { Metadata } from 'next'
import '@/styles/global.scss'

const currentEnv = process.env.ENV;

export const metadata: Metadata = {
  title: `Todo App ${currentEnv}`,
  description: 'A simple todo app'
}

export default function RootLayout({
  children
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en" className="dark">
      <body>{children}</body>
    </html>
  )
}
