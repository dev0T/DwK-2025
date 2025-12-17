import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server'

export function proxy(request: NextRequest) {

  const startTime = Date.now()
  const { method, url, nextUrl } = request
  
  console.log(`[${new Date().toISOString()}] ${method} ${nextUrl.pathname}`)
  
  const requestHeaders = new Headers(request.headers)
  requestHeaders.set('x-request-id', crypto.randomUUID())
  requestHeaders.set('x-start-time', startTime.toString())
  
  const response = NextResponse.next({
    request: {
      headers: requestHeaders,
    },
  })
  
  response.headers.set('x-response-time', `${Date.now() - startTime}ms`)
}