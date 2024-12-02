import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { OnchainVoting } from '../target/types/onchain_voting';
import { PublicKey } from '@solana/web3.js';

describe('onchain-voting', async () => {
	// Configure the client to use the local cluster.
	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.OnchainVoting as Program<OnchainVoting>;

	let voteBank = anchor.web3.Keypair.generate();

	const [pda, bump] = await PublicKey.findProgramAddressSync(
		[Buffer.from('seeds')],
		program.programId
	);

	it('Initialize vote registry', async () => {
		program.account.voteRegistry
			.fetch(pda)
			.then(() => {
				console.log(
					'Vote Registry PDA already initialized! PDA ::',
					pda.toString()
				);
			})
			.catch(async () => {
				const tx = await program.methods.initVoteRegistry().rpc();
				console.log('Vote Registry Initialized :: TxHash ::', tx);
			});
	});

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

			console.log(
				'Created Vpte Bank for Public to Vote :: TxHash ::',
				tx
			);
		} else {
			console.log('Voting account exists! Proceed to cast your vote!');
		}
	});

	it('Add Option to Vote Account', async () => {
		const tx = await program.methods
			.addOptionToVote('Option One')
			.accounts({
				voteAccount: voteBank.publicKey,
			})
			.rpc();

		console.log('Option Added to Vote Account :: TxHash ::', tx);
	});

	it('Vote for GM', async () => {
		const vote_option_id = new anchor.BN(1);
		const tx = await program.methods
			.castVote(vote_option_id)
			.accounts({
				voteAccount: voteBank.publicKey,
			})
			.rpc();

		console.log('Vote Cast for First Vote Option :: TxHash ::', tx);
	});

	const pda_data = await program.account.voteRegistry.fetch(pda);
	console.log(JSON.stringify(pda_data, null, 4));
});
