import type { CrateDependency } from './index';

/**
 * Converts a CrateDependency to a TOML-formatted dependency string
 *
 * @param crateName - The crate name (e.g., "yellowstone-grpc-client")
 * @param dependency - The dependency definition (string or object)
 * @param additionalFeatures - Optional features to add/override
 * @param optional - Whether the dependency should be marked as optional
 * @returns TOML-formatted string like: `crate = { git = "...", rev = "..." }`
 */
export function getCrateDependencyString(
    crateName: string,
    dependency: CrateDependency,
    additionalFeatures?: string[],
    optional?: boolean,
): string {
    if (typeof dependency === 'string') {
        if (optional || (additionalFeatures && additionalFeatures.length > 0)) {
            const parts: string[] = [`version = "${dependency}"`];
            if (additionalFeatures && additionalFeatures.length > 0) {
                const featuresStr = additionalFeatures.map(f => `"${f}"`).join(', ');
                parts.push(`features = [${featuresStr}]`);
            }
            if (optional) {
                parts.push('optional = true');
            }
            return `${crateName} = { ${parts.join(', ')} }`;
        }
        return `${crateName} = "${dependency}"`;
    }

    const parts: string[] = [];

    if (dependency.version) {
        parts.push(`version = "${dependency.version}"`);
    }

    if (dependency.git) {
        parts.push(`git = "${dependency.git}"`);
    }

    if (dependency.rev) {
        parts.push(`rev = "${dependency.rev}"`);
    }

    if (dependency.branch) {
        parts.push(`branch = "${dependency.branch}"`);
    }

    const allFeatures = [...(dependency.features || []), ...(additionalFeatures || [])];
    if (allFeatures.length > 0) {
        const featuresStr = allFeatures.map(f => `"${f}"`).join(', ');
        parts.push(`features = [${featuresStr}]`);
    }

    if (dependency.defaultFeatures === false) {
        parts.push('default-features = false');
    }

    if (optional) {
        parts.push('optional = true');
    }

    return `${crateName} = { ${parts.join(', ')} }`;
}
