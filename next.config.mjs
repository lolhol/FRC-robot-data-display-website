/** @type {import('next').NextConfig} */
const nextConfig = {
  experimental: {
    instrumentationHook: true,
  },
  async rewrites() {
    return [
      {
        source: "/api/database/:path*",
        destination: `http://localhost:${process.env.RUST_DB_API_PORT}/:path*`, // Read port from .env
      },
    ];
  },
};

// Use ES module syntax to export the config
export default nextConfig;
