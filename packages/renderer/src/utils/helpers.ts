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
