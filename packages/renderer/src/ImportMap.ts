import { TypeManifest } from './getTypeManifestVisitor';

export class ImportMap {
    protected readonly _imports: Set<string> = new Set();

    get imports(): Set<string> {
        return this._imports;
    }

    add(imports: Set<string> | string[] | string): ImportMap {
        const newImports = typeof imports === 'string' ? [imports] : imports;
        newImports.forEach(i => this._imports.add(i));
        return this;
    }

    mergeWith(...others: ImportMap[]): ImportMap {
        others.forEach(other => {
            this.add(other._imports);
        });
        return this;
    }

    mergeWithManifest(manifest: TypeManifest): ImportMap {
        return this.mergeWith(manifest.imports);
    }

    toString(): string {
        return [...this._imports]
            .sort()
            .map(i => `use ${i};`)
            .join('\n');
    }
}
