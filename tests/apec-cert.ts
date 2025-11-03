import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { ApecCert } from "../target/types/apec_cert";
import { expect } from "chai";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

import { MerkleTree } from "merkletreejs";
import { keccak_256 } from "@noble/hashes/sha3";

describe("apec-cert", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet;

  const program = anchor.workspace.apecCert as Program<ApecCert>;

  const providerParams = {
    id: 1,
    shortName: "APEC",
  };

  const courseParams = {
    id: 1,
    shortName: "Blockchain 101",
  };

  const certParams = {
    name: "Blockchain 101 Certificate",
    uri: "https://example.com/certificates/blokchain101.json",
  };

  const [providerPda, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("provider"),
      new BN(providerParams.id).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [coursePda, __] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("course"),
      providerPda.toBuffer(),
      new BN(courseParams.id).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );
  const student = anchor.web3.Keypair.generate();
  const student2 = anchor.web3.Keypair.generate();
  const student3 = anchor.web3.Keypair.generate();
  const student4 = anchor.web3.Keypair.generate();

  const certList = [
    payer.publicKey,
    student.publicKey,
    student2.publicKey,
    student3.publicKey,
    student4.publicKey,
  ];

  const [certProofPda, ___] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("cert_proof"), providerPda.toBuffer(), coursePda.toBuffer()],
    program.programId
  );

  const getMerkleTree = (data: Uint8Array[]): MerkleTree => {
    return new MerkleTree(data.map(keccak_256), keccak_256, {
      sortPairs: true,
    });
  };

  const getMerkleRoot = (data: Uint8Array[]): Uint8Array => {
    return getMerkleTree(data).getRoot();
  };

  const getMerkleProof = (
    data: Uint8Array[],
    leaf: Uint8Array,
    index?: number
  ): Uint8Array[] => {
    return getMerkleTree(data)
      .getProof(Buffer.from(keccak_256(leaf)), index)
      .map((proofItem) => proofItem.data);
  };

  const merkleTreeRoot = getMerkleRoot(certList.map((x) => x.toBuffer()));

  it("Init Provider", async () => {
    const tx = await program.methods
      .initProvider(new BN(providerParams.id), providerParams.shortName)
      .accounts({
        authority: payer.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    const providerAccount = await program.account.provider.fetch(providerPda);
    expect(providerAccount.id.toNumber()).to.equal(providerParams.id);
    expect(providerAccount.shortName).to.equal(providerParams.shortName);
  });

  it("Create Course", async () => {
    const tx = await program.methods
      .createCourse(new BN(courseParams.id), courseParams.shortName)
      .accountsPartial({
        provider: providerPda,
        authority: payer.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    // Add assertions to verify course creation if needed
    const courseAccount = await program.account.course.fetch(coursePda);
    expect(courseAccount.id.toNumber()).to.equal(courseParams.id);
    expect(courseAccount.shortName).to.equal(courseParams.shortName);
  });

  it("Enroll Course", async () => {
    const tx = await program.methods
      .enrollCourse(
        "Blockchain 101 Certificate",
        "BLK101",
        "https://example.com/certificates/blokchain101.json"
      )
      .accountsPartial({
        user: payer.publicKey,
        course: coursePda,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    // Add assertions to verify enrollment if needed
  });

  it("Create cert merkle tree proof", async () => {
    console.log(merkleTreeRoot);
    const tx = await program.methods
      .createCertProof(merkleTreeRoot as any, new BN(certList.length))
      .accountsPartial({
        provider: providerPda,
        course: coursePda,
        authority: payer.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Enroll with cert proof", async () => {
    const proof = getMerkleProof(
      certList.map((x) => x.toBuffer()),
      payer.publicKey.toBuffer()
    );

    const tx = await program.methods
      .claimCert(proof as any, certParams.name, certParams.uri)
      .accountsPartial({
        claimer: payer.publicKey,
        provider: providerPda,
        course: coursePda,
        certProof: certProofPda,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
