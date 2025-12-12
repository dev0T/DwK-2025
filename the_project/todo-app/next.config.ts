import type { NextConfig } from "next";

const envBasePath = (): string => {
  const currentEnv = process.env.ENV;
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
  serverExternalPackages: ['pino'],
};

export default nextConfig;
