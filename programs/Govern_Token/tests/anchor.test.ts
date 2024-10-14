describe("Test Minter", () => {
  // Metaplex Constants
  const METADATA_SEED = "metadata";
  const TOKEN_METADATA_PROGRAM_ID = new web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  // Constants from our program
  const MINT_SEED = "govern token";

  // Data for our tests
  const payer = pg.wallet.publicKey;
  const metadata = {
    name: "Govern Token",
    symbol: "GTN",
    uri: "https://turquoise-impossible-owl-140.mypinata.cloud/ipfs/QmYsWuTSytYHWwg4ryqBYdvfXq6s2SfWEQxowqyjpcVHqJ",
    decimals: 6,
  };
  const mintAmount = 10;
  const [mint] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from(MINT_SEED)],
    pg.PROGRAM_ID
  );
  const test_wallet = anchor.utils.token.associatedAddress({
    mint: mint,
    owner: new web3.PublicKey("4Mf83WqpB2e34smSRjcKVbyHeQ3EmLvicvM4gLMCLj1V"), // Replace with actual recipient address
  });

  const [metadataAddress] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(METADATA_SEED),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  // Test init token
  it("initialize", async () => {
    const info = await pg.connection.getAccountInfo(mint);
    if (info) {
      return; // Do not attempt to initialize if already initialized
    }
    console.log("  Mint not found. Attempting to initialize.");

    const context = {
      metadata: metadataAddress,
      mint,
      payer,
      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    };

    const tx = await pg.program.methods
      .initToken(metadata)
      .accounts(context)
      .transaction();

    const txHash = await web3.sendAndConfirmTransaction(
      pg.connection,
      tx,
      [pg.wallet.keypair],
      { skipPreflight: true }
    );
    console.log(`  https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
    const newInfo = await pg.connection.getAccountInfo(mint);
    assert(newInfo, "  Mint should be initialized.");
  });

  // Test mint tokens
  it("mint tokens", async () => {
    const destination = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: payer,
    });

    let initialBalance: number;
    try {
      const balance = await pg.connection.getTokenAccountBalance(destination);
      initialBalance = balance.value.uiAmount;
    } catch {
      // Token account not yet initiated has 0 balance
      initialBalance = 0;
    }

    const context = {
      mint,
      destination,
      payer,
      rent: web3.SYSVAR_RENT_PUBKEY,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    };

    const tx = await pg.program.methods
      .mintToken(new BN(mintAmount * 10 ** metadata.decimals))
      .accounts(context)
      .transaction();
    const txHash = await web3.sendAndConfirmTransaction(
      pg.connection,
      tx,
      [pg.wallet.keypair],
      { skipPreflight: true }
    );
    console.log(`  https://explorer.solana.com/tx/${txHash}?cluster=devnet`);

    const postBalance = (
      await pg.connection.getTokenAccountBalance(destination)
    ).value.uiAmount;
    assert.equal(
      initialBalance + mintAmount,
      postBalance,
      "Post balance should equal initial plus mint amount"
    );
  });

  // Test transfer tokens
  it("transfer tokens", async () => {
    const from = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: pg.wallet.publicKey,
    });

    const toOwner = new web3.PublicKey(
      "38dUEwwfdTMPdv2t3sXqw5U8UBF4SGie3d6sqPXvZatq"
    );
    const to = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: toOwner,
    });

    let sourceBalance: number;
    try {
      sourceBalance = (await pg.connection.getTokenAccountBalance(from)).value
        .uiAmount;
    } catch {
      sourceBalance = 0;
    }
    console.log("Source Balance:", sourceBalance);

    let destinationBalance: number;
    try {
      destinationBalance = (await pg.connection.getTokenAccountBalance(to))
        .value.uiAmount;
    } catch {
      destinationBalance = 0;
    }
    console.log("Destination Balance:", destinationBalance);

    const transferAmount = 5;

    const context = {
      mint,
      from,
      to,
      payer,
      toOwner,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    };

    const tx = await pg.program.methods
      .transferToken(new BN(transferAmount * 10 ** metadata.decimals))
      .accounts(context)
      .transaction();

    const txHash = await web3.sendAndConfirmTransaction(
      pg.connection,
      tx,
      [pg.wallet.keypair],
      { skipPreflight: true }
    );
    console.log(`  https://explorer.solana.com/tx/${txHash}?cluster=devnet`);

    const sourcePostBalance = (await pg.connection.getTokenAccountBalance(from))
      .value.uiAmount;
    const destinationPostBalance = (
      await pg.connection.getTokenAccountBalance(to)
    ).value.uiAmount;

    console.log("Source Post Balance:", sourcePostBalance);
    console.log("Destination Post Balance:", destinationPostBalance);

    assert.equal(
      sourceBalance - transferAmount,
      sourcePostBalance,
      "Source balance should decrease by transfer amount"
    );
    assert.equal(
      destinationBalance + transferAmount,
      destinationPostBalance,
      "Destination balance should equal transfer amount + destination Balance"
    );
  });

  // // Test transfer checked tokens
  // it("transfer checked tokens", async () => {
  //   const from = await anchor.utils.token.associatedAddress({
  //     mint: mint,
  //     owner: payer,
  //   });

  //   const to = test_wallet;

  //   let sourceBalance = (await pg.connection.getTokenAccountBalance(from)).value
  //     .uiAmount;
  //   console.log("Source Balance:", sourceBalance);

  //   const transferAmount = 5;

  //   const context = {
  //     mint,
  //     from,
  //     to,
  //     payer,
  //     systemProgram: web3.SystemProgram.programId,
  //     tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
  //     associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
  //   };

  //   const tx = await pg.program.methods
  //     .transferCheckedToken(new BN(transferAmount * 10 ** metadata.decimals))
  //     .accounts(context)
  //     .transaction();

  //   const txHash = await web3.sendAndConfirmTransaction(
  //     pg.connection,
  //     tx,
  //     [pg.wallet.keypair],
  //     { skipPreflight: true }
  //   );
  //   console.log(`  https://explorer.solana.com/tx/${txHash}?cluster=devnet`);

  //   const sourcePostBalance = (await pg.connection.getTokenAccountBalance(from))
  //     .value.uiAmount;
  //   const destinationPostBalance = (
  //     await pg.connection.getTokenAccountBalance(to)
  //   ).value.uiAmount;

  //   console.log("Source Post Balance:", sourcePostBalance);
  //   console.log("Destination Post Balance:", destinationPostBalance);

  //   assert.equal(
  //     sourceBalance - transferAmount,
  //     sourcePostBalance,
  //     "Source balance should decrease by transfer amount"
  //   );
  //   assert.equal(
  //     destinationPostBalance,
  //     transferAmount,
  //     "Destination balance should equal transfer amount"
  //   );
  // });

  // Test burn tokens
  it("burn tokens", async () => {
    const source = await anchor.utils.token.associatedAddress({
      mint: mint,
      owner: payer,
    });

    let initialBalance = (await pg.connection.getTokenAccountBalance(source))
      .value.uiAmount;
    console.log("Initial Balance before burn:", initialBalance);

    const burnAmount = 3;

    const context = {
      mint,
      from: source,
      payer,
      systemProgram: web3.SystemProgram.programId,
      tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
    };

    const tx = await pg.program.methods
      .burnToken(new BN(burnAmount * 10 ** metadata.decimals))
      .accounts(context)
      .transaction();

    const txHash = await web3.sendAndConfirmTransaction(
      pg.connection,
      tx,
      [pg.wallet.keypair],
      { skipPreflight: true }
    );
    console.log(`  https://explorer.solana.com/tx/${txHash}?cluster=devnet`);

    const postBurnBalance = (await pg.connection.getTokenAccountBalance(source))
      .value.uiAmount;

    console.log("Post Burn Balance:", postBurnBalance);

    assert.equal(
      initialBalance - burnAmount,
      postBurnBalance,
      "Balance should decrease by burn amount"
    );
  });
});
