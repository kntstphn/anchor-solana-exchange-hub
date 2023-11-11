import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorExchangeHub } from "../target/types/anchor_exchange_hub";
import { assert } from "chai";

describe("anchor-exchange-hub", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const keyPair = anchor.web3.Keypair.generate();

  const program = anchor.workspace
    .AnchorExchangeHub as Program<AnchorExchangeHub>;

  it("Create a post", async () => {
    const topic = "Solana Topic";
    const content = "Solana Post";

    await program.methods
      .createPost(topic, content)
      .accounts({
        author: provider.publicKey,
        post: keyPair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([keyPair])
      .rpc();

    const posted = await program.account.post.fetch(keyPair.publicKey);

    assert.equal(
      posted.author.toBase58(),
      program.provider.publicKey.toBase58()
    );
    assert.ok(topic == posted.topic);
    assert.ok(content == posted.content);
    assert.ok(posted.timestamp);
  });

  // it("Upvote a post", async () => {
  //   const topic = "Solana Topic";
  //   const content = "Solana Post";

  //   await program.methods
  //     .createPost(topic, content)
  //     .accounts({
  //       author: provider.publicKey,
  //       post: keyPair.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .signers([keyPair])
  //     .rpc();

  //   const posted = await program.account.post.fetch(keyPair.publicKey);
  //   console.log("Posted:", posted);

  //   await program.methods
  //     .upvotePost()
  //     .accounts({
  //       post: provider.publicKey,
  //       user: keyPair.publicKey,
  //     })
  //     .signers([keyPair])
  //     .rpc();

  //   const updatedPost = await program.account.post.fetch(provider.publicKey);
  //   console.log("Updated Post:", updatedPost);

  //   console.log("Expected Upvotes:", posted.upvotes.toNumber() + 1);
  //   console.log("Actual Upvotes:", updatedPost.upvotes.toNumber());

  //   assert.equal(updatedPost.upvotes.toNumber(), posted.upvotes.toNumber() + 1);
  // });
});
