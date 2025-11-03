import type { CrateDependency } from './index';

/**
 * Converts a CrateDependency to a TOML-formatted dependency string
 * 
 * @param crateName - The crate name (e.g., "yellowstone-grpc-client")
 * @param dependency - The dependency definition (string or object)
 * @param additionalFeatures - Optional features to add/override
 * @returns TOML-formatted string like: `crate = { git = "...", rev = "..." }`
 */
export function getCrateDependencyString(
    crateName: string,
    dependency: CrateDependency,
    additionalFeatures?: string[]
): string {
    if (typeof dependency === 'string') {
        if (additionalFeatures && additionalFeatures.length > 0) {
            const featuresStr = additionalFeatures.map(f => `"${f}"`).join(', ');
            return `${crateName} = { version = "${dependency}", features = [${featuresStr}] }`;
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
    
    if (dependency.path) {
        parts.push(`path = "${dependency.path}"`);
    }
    
    const allFeatures = [
        ...(dependency.features || []),
        ...(additionalFeatures || [])
    ];
    if (allFeatures.length > 0) {
        const featuresStr = allFeatures.map(f => `"${f}"`).join(', ');
        parts.push(`features = [${featuresStr}]`);
    }
    
    if (dependency.defaultFeatures === false) {
        parts.push('default-features = false');
    }
    
    return `${crateName} = { ${parts.join(', ')} }`;
}

