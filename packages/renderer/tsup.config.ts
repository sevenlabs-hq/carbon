import { defineConfig } from 'tsup';

export default defineConfig({
    entry: ['src/index.ts'],
    format: ['cjs', 'esm'],
    dts: true,
    splitting: false,
    sourcemap: true,
    clean: true,
    minify: false,
    target: 'es2022',
    outDir: 'dist',
    define: {
        __ESM__: 'true',
        __TEST__: 'false',
        __VERSION__: '"1.0.0"',
    },
});
