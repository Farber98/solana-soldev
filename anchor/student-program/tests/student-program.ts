import { StudentProgram } from './../target/types/student_program';
import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { assert, expect } from "chai"

describe("student-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)


  const program = anchor.workspace.StudentProgram as Program<StudentProgram>;

  const studentGreeting = {
    student: "Juan Farber",
    message: "Hey Anchor!",
  }

  const [studentPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [provider.wallet.publicKey.toBuffer(), Buffer.from(studentGreeting.student)],
    program.programId
  )

  it("Student greeting is added`", async () => {
    const tx = await program.methods
      .addStudentGreeting(studentGreeting.student, studentGreeting.message)
      .accounts({ studentGreeting: studentPDA })
      .rpc()

    const account = await program.account.studentAccountState.fetch(studentPDA)
    expect(studentGreeting.student === account.student)
    expect(studentGreeting.message === account.message)
    expect(account.user === provider.wallet.publicKey)
  })

  it("Student greeting is updated`", async () => {
    const newMessage = "Hey Solana with Anchor!"

    const tx = await program.methods
      .updateStudentGreeting(studentGreeting.student, newMessage)
      .accounts({ studentGreeting: studentPDA })
      .rpc()

    const account = await program.account.studentAccountState.fetch(studentPDA)
    expect(studentGreeting.student === account.student)
    expect(newMessage === account.message)
    expect(account.user === provider.wallet.publicKey)

  })

  it("Deletes a student greeting", async () => {
    const tx = await program.methods
      .deleteStudentGreeting(studentGreeting.student)
      .accounts({ studentGreeting: studentPDA })
      .rpc()
  })
});
