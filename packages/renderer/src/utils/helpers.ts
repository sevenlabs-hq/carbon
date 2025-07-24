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
