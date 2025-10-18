import { defineConfig } from 'tsup';

export default defineConfig({
    entry: ['src/cli.ts', 'src/index.ts'],
    format: ['cjs', 'esm'],
    dts: true,
    sourcemap: true,
    clean: true,
    target: 'es2022',
    outDir: 'dist',
    banner: {
        js: '#!/usr/bin/env node',
    },
});
