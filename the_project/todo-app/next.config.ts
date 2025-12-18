import type { NextConfig } from "next";

const envBasePath = (): string => {
  const currentEnv = process.env.ENV;
  console.log("currentEnv: ", currentEnv)
  if (currentEnv) {
    switch (currentEnv) {
      case "production":
        return "";
      default:
        return `/${currentEnv}`
      }
  }
  return ""
}

const nextConfig: NextConfig = {
  basePath: envBasePath(),
  output: "standalone",
  serverExternalPackages: ['pino', 'next-logger'],
  logging: {
    fetches: {
      fullUrl: true,
    },
    incomingRequests: true
  },
  experimental: {
    globalNotFound: true,
  },
};

export default nextConfig;
