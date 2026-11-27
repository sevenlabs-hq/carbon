import { isNode, resolveNestedTypeNode, type NumberFormat, type TypeNode } from '@codama/nodes';

export function partition<T>(
    arr: readonly T[],
    predicate: (value: T, index: number, array: readonly T[]) => boolean,
): [T[], T[]] {
    return arr.reduce<[T[], T[]]>(
        (out, v, i, a) => {
            out[predicate(v, i, a) ? 0 : 1].push(v);
            return out;
        },
        [[], []],
    );
}

export type TokenProgramKind = 'spl-token' | 'token-2022';

const TOKEN_PROGRAM_KINDS: Record<string, TokenProgramKind> = {
    TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA: 'spl-token',
    TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb: 'token-2022',
};

/** Token programs use the SPL `Pack` layout, not Borsh, so they get program-specific code. */
export function tokenProgramKind(programId?: string | null): TokenProgramKind | undefined {
    return programId ? TOKEN_PROGRAM_KINDS[programId] : undefined;
}

/** Format of an `Option<int>` newtype such as pump.fun's `OptionU64`. */
export function optionalNumberNewtypeFormat(typeName: string): NumberFormat | undefined {
    return /^option(u8|u16|u32|u64|i8|i16|i32|i64)$/.exec(typeName.toLowerCase())?.[1] as NumberFormat | undefined;
}

export type SizePrefixNewtype = 'U64PrefixString' | 'U64PrefixBytes' | 'U8PrefixBytes';

/** Core newtype for a size prefix that borsh's own layout reads at the wrong width. */
export function sizePrefixNewtype(typeNode: TypeNode): SizePrefixNewtype | undefined {
    if (!isNode(typeNode, 'sizePrefixTypeNode')) return undefined;
    const width = resolveNestedTypeNode(typeNode.prefix).format;
    if (width === 'u64' && isNode(typeNode.type, 'stringTypeNode')) return 'U64PrefixString';
    if (width === 'u64' && isNode(typeNode.type, 'bytesTypeNode')) return 'U64PrefixBytes';
    if (width === 'u8' && isNode(typeNode.type, 'bytesTypeNode')) return 'U8PrefixBytes';
    return undefined;
}
