import type { Metadata } from 'next'
import '@/styles/global.scss'

export const metadata: Metadata = {
  title: 'Todo App',
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
