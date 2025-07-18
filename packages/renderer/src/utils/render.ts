import { dirname as pathDirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { camelCase, kebabCase, pascalCase, snakeCase, titleCase } from '@codama/nodes';
import nunjucks, { ConfigureOptions as NunJucksOptions } from 'nunjucks';

export function rustDocblock(docs: string[]): string {
    if (docs.length <= 0) return '';
    const lines = docs.map(doc => `/// ${doc}`);
    return `${lines.join('\n')}\n`;
}

export const render = (template: string, context?: object, options?: NunJucksOptions): string => {
    // Get templates directory from the package directory
    const isESM = typeof import.meta !== 'undefined';
    const dirname = isESM ? pathDirname(fileURLToPath(import.meta.url)) : __dirname;
    const templates = join(dirname, '..', 'templates'); // Path to templates from src/utils

    const env = nunjucks.configure(templates, { autoescape: false, trimBlocks: true, ...options });
    env.addFilter('pascalCase', pascalCase);
    env.addFilter('camelCase', camelCase);
    env.addFilter('snakeCase', snakeCase);
    env.addFilter('kebabCase', kebabCase);
    env.addFilter('titleCase', titleCase);
    env.addFilter('rustDocblock', rustDocblock);
    return env.render(template, context);
};
