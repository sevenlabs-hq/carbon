import * as anchor from '@coral-xyz/anchor';
import { exitWithError, resolveRpcUrl } from './utils';

export async function fetchAnchorIdl(programAddress: string, rpcUrl: string): Promise<any> {
    try {
        const connection = new anchor.web3.Connection(resolveRpcUrl(rpcUrl), 'confirmed');
        const programId = new anchor.web3.PublicKey(programAddress);
        const idl = await anchor.Program.fetchIdl(programId, { connection });
        if (!idl) {
            exitWithError('No Anchor IDL found for program address.');
        }
        return idl;
    } catch (e: any) {
        exitWithError(`Failed to fetch Anchor IDL: ${e?.message ?? String(e)}`);
    }
}
