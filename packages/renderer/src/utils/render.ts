import { join } from 'node:path';

import { camelCase, kebabCase, pascalCase, snakeCase, titleCase } from '@codama/nodes';
import nunjucks, { ConfigureOptions as NunJucksOptions } from 'nunjucks';
import { RUST_KEYWORDS, escapeRustKeyword } from '../constants/rustKeywords';

export function rustDocblock(docs: string[]): string {
    if (docs.length <= 0) return '';
    const lines = docs.map(doc => `/// ${doc}`);
    return `${lines.join('\n')}\n`;
}

// IDL docs sometimes close a one-backtick code span with two backticks, which leaves the span
// open to the end of the line; close it with one. Only spans without spaces are repaired, so a
// stray backtick before a real double-backtick span is left alone.
function closeCodeSpans(line: string): string {
    return line.replace(/(?<!`)`([^`\s]+)``(?!`)/g, '`$1`');
}

export function formatDocComments(docs: string[], indent: string = ''): string {
    if (docs.length === 0) {
        return '';
    }
    // Process all docs together as one unit to maintain list context across doc strings
    // This is critical for proper indentation of continuation lines after list items
    const allLines = docs.join('\n').split('\n');
    const result: string[] = [];
    const isBulletLine = (s: string): boolean => /^\* +/.test(s) || /^- +/.test(s) || /^[0-9]+\. +/.test(s);

    let inCodeBlock = false;

    for (let i = 0; i < allLines.length; i++) {
        const line = allLines[i];
        const text = line.trim();

        // Skip empty lines completely - never output them
        if (text === '') {
            continue;
        }

        // Code blocks are left as written.
        const isFence = text.startsWith('```');
        if (isFence) {
            inCodeBlock = !inCodeBlock;
        }
        const trimmed = isFence || inCodeBlock ? text : closeCodeSpans(text);

        const isListItem = isBulletLine(trimmed);

        if (isListItem) {
            // List item - output trimmed version, set context to true
            result.push(`${indent}/// ${trimmed}`);
        } else {
            // Not a list item; this may follow a list or start a new section/paragraph.
            // When a non-list line directly follows a list item, Clippy expects a blank
            // doc line to mark a paragraph break; otherwise it flags doc_lazy_continuation.
            let prevNonEmpty: string | null = null;
            for (let j = i - 1; j >= 0; j--) {
                const prevTrimmed = allLines[j].trim();
                if (prevTrimmed !== '') {
                    prevNonEmpty = prevTrimmed;
                    break;
                }
            }

            if (prevNonEmpty) {
                const isPrevBullet = isBulletLine(prevNonEmpty);
                if (isPrevBullet) {
                    // Blank doc line (paragraph break)
                    result.push(`${indent}///`);
                }
            }

            result.push(`${indent}/// ${trimmed}`);
        }
    }

    // Join with newlines - no trailing newline to avoid empty_line_after_doc_comment errors
    return result.join('\n');
}

export const render = (template: string, context?: object, options?: NunJucksOptions): string => {
    const templates = join(__dirname, '..', 'templates');

    const env = nunjucks.configure(templates, { autoescape: false, trimBlocks: true, ...options });

    env.addFilter('pascalCase', pascalCase);
    env.addFilter('camelCase', camelCase);
    env.addFilter('snakeCase', snakeCase);
    env.addFilter('kebabCase', kebabCase);
    env.addFilter('titleCase', titleCase);
    env.addFilter('rustDocblock', rustDocblock);
    env.addFilter('formatDocComments', (docs: string[], indent: string = '') => formatDocComments(docs, indent));
    env.addFilter('closeCodeSpans', closeCodeSpans);
    env.addFilter('escapeRustKeyword', escapeRustKeyword);

    env.addGlobal('RUST_KEYWORDS', RUST_KEYWORDS);

    return env.render(template, context);
};
