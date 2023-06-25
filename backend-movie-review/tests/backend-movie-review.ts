import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BackendMovieReview } from "../target/types/backend_movie_review";
import { Guid } from "guid-typescript";

describe("backend-movie-review", () => 
{
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.BackendMovieReview as Program<BackendMovieReview>;
  const title = "RRR";
  const description = "Best Indian Movie ever!";
  const review = 5;
  const comment = "So True!";
  const commentID = "ID"; 
  const [movieAccountPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("movie_account"), provider.publicKey.toBuffer(), Buffer.from(title)], program.programId);
  
  it("Add Movie", async () => 
  {
    await program.methods.addMovie(title, description, review).rpc();
    const movieAccount = await program.account.movieAccount.fetch(movieAccountPda);
    
    console.log("Movie Account: ");
    console.log(movieAccount.title);
    console.log(movieAccount.description);
    console.log(movieAccount.review);
    console.log(movieAccount.comments);
  });

  it("Update Movie", async () => 
  {
    await program.methods.editMovie(title, "Best Movie ever!", review).rpc();
    const movieAccount = await program.account.movieAccount.fetch(movieAccountPda);
    
    console.log("Movie Account: ");
    console.log(movieAccount.title);
    console.log(movieAccount.description);
    console.log(movieAccount.review);
    console.log(movieAccount.comments);
  });

  it("Add Comment", async () => 
  { 
    const [commentAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("comment_account"), provider.publicKey.toBuffer(), Buffer.from(commentID.toString()), movieAccountPda.toBuffer()], program.programId);

    await program.methods.addComment(title, comment, commentID.toString()).rpc();
    const commentAccount = await program.account.commentAccount.fetch(commentAccountPda);
    const movieAccount = await program.account.movieAccount.fetch(movieAccountPda);

    console.log("Comment Account: ");
    console.log(commentAccount.comment);
    console.log("Movie Account: ");
    console.log(movieAccount.title);
    console.log(movieAccount.description);
    console.log(movieAccount.review);
    console.log(movieAccount.comments);
  });

  it("Edit Comment", async () => 
  {
    const [commentAccountPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("comment_account"), provider.publicKey.toBuffer(), Buffer.from(commentID.toString()), movieAccountPda.toBuffer()], program.programId);

    await program.methods.editComment(title, "EVER!!!!", commentID.toString()).rpc();

    const commentAccount = await program.account.commentAccount.fetch(commentAccountPda);

    console.log("Comment Account: ");
    console.log(commentAccount.comment);
  });

  it("Close Comment", async () => 
  {
    await program.methods.closeComment(title, commentID.toString()).rpc();

    const movieAccount = await program.account.movieAccount.fetch(movieAccountPda);
    
    console.log("Movie Account: ");
    console.log(movieAccount.title);
    console.log(movieAccount.description);
    console.log(movieAccount.review);
    console.log(movieAccount.comments);
  });
});
