const removeImports = require('next-remove-imports')();

/** @type {import('next').NextConfig} */
module.exports = removeImports({
  reactStrictMode: true,
  webpack: (config) => {
    config.output.webassemblyModuleFilename = 'static/wasm/[modulehash].wasm';
    config.experiments = { asyncWebAssembly: true };
    return config;
  },
  experimental: { esmExternals: true },
});
