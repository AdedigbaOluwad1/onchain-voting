import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { OnchainVoting } from '../target/types/onchain_voting';

describe('onchain-voting', () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.OnchainVoting as Program<OnchainVoting>;

	const voteBankSecret = new Uint8Array([
		164, 57, 122, 92, 121, 11, 202, 246, 100, 113, 178, 197, 17, 113, 165,
		75, 198, 205, 107, 159, 84, 96, 171, 130, 83, 212, 148, 60, 219, 121,
		243, 157, 6, 251, 206, 84, 255, 40, 118, 152, 31, 121, 211, 205, 51,
		233, 134, 77, 134, 36, 246, 98, 152, 139, 63, 216, 237, 193, 25, 53,
		222, 99, 30, 56,
	]);

	let voteBank = anchor.web3.Keypair.generate()
  // .fromSecretKey(voteBankSecret);

	it('Creating vote bank for public to vote', async () => {
		let isAccountInitialized: boolean;

		await program.account.voteBank
			.fetch(voteBank.publicKey)
			.then(() => {
				isAccountInitialized = true;
			})
			.catch(() => (isAccountInitialized = false));

		if (!isAccountInitialized) {
			const tx = await program.methods
				.createVoteBank()
				.accounts({
					voteAccount: voteBank.publicKey,
				})
				.signers([voteBank])
				.rpc();
			console.log('TxHash ::', tx);
		} else {
			console.log('Voting account exists! Proceed to cast your vote!');
		}
	});

	it('Vote for GM', async () => {
		const tx = await program.methods
			.castVote({ gn: {} })
			.accounts({
				voteAccount: voteBank.publicKey,
			})
			.rpc();
		console.log('TxHash ::', tx);

		let voteBankData = await program.account.voteBank.fetch(
			voteBank.publicKey
		);
		console.log(`Bank Data ${JSON.stringify(voteBankData, null, 2)}`);
	});
});
