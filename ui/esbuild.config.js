const esbuild = require('esbuild');
const path = require('path');

esbuild.build({
  entryPoints: ['ts/main.ts'],
  bundle: true,
  outdir: 'dist',
  target: 'ES2016', // Better cross-browser support (includes most modern browsers)
  format: 'iife',
  sourcemap: false,
  minify: true,
  platform: 'browser',
}).catch(() => process.exit(1));
