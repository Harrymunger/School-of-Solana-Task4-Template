
![School of Solana](https://github.com/School-of-Solana/.github/blob/main/assets/Season-5-Banner.png?raw=true)

## 📚Task 4
This is your **Task 4** in the **School of Solana Season 5**. During the previous task, you were introduced to the basics of Solana programming. The goal of this week's task is to further familiarize you with advanced development workflows. In this task, you will become acquainted with concepts such as PDAs, common project structures, proper storage of variable data within accounts, and more.

### Task details
This week's task is similar to a well-known Twitter App. The program can perform basic functions like **creating tweets**, **adding/removing tweet reactions** (like/dislike, you can extend it freely, but remember new reactions are not part of evaluation), and **adding/removing tweet comments**.

Here's how it works:

1. Someone wants to **create a tweet**. The tweet includes a topic and content. The topic can be up to **32 bytes** in size, and the content can be up to **500 bytes**. The topic serves as one of the seeds for a PDA (for the Tweet Account), ensuring that a user can create more than one tweet.

2. When someone wants to **add a reaction** to a tweet, the program creates a new reaction account (which also has a PDA) and stores important data in it. The seeds for the PDA are chosen to prevent more than one reaction per user on one tweet.

3. Lastly, there are **comments**. A comment includes various fields. One of the variables is called content, which is also limited to a length of **500 bytes**, and it stores the text content of the comment. The content field is also used as input into the PDA of the comment account. You might wonder how we managed to use content that is **500 bytes** long as part of the PDA seed, given that PDA seeds are limited to 32 bytes. You can find the answer in the source code, or you can discuss it on Discord.

**Your task is to understand how the program works and implement all parts marked as TODO, start within the `lib.rs`**.

**If you have any questions, or you do not understand anything, feel free to ask on Discord!**


### Submission Process
The source code of the on-chain program is stored within the `programs/twitter` folder, in this folder you can also find your TODOs.

**Please do not commit any other changes than those made within the `programs/twitter` folder, as it will make the evaluation process more difficult.**

### Deadline
The deadline for this task is **Wednesday, May 15th, at 23:59 UTC**.

### Evaluation
We will evaluate your submission using the same test suite provided in this task. Therefore, the requirements for this task are to pass **100%** of the provided tests.

### Setup
For this Task you need:
- [Rust installed](https://www.rust-lang.org/tools/install)
    - Make sure to use stable version:
    ```
    rustup default stable
    ```
- [Solana installed](https://docs.solana.com/cli/install-solana-cli-tools)
    - Use v1.18.11

- [Anchor installed](https://www.anchor-lang.com/docs/installation)
    - Use v0.29.0

### Commands
With the setup I described above, you should be able to run the following commands.

1. You should have **Yarn** installed as it is one of the steps during **Anchor** installation, so once you clone the repo, you should be able to run:
    ```
    yarn install
    ```

2. To build the project, run:
    ```
    anchor build
    ```

3. To test the project, run:
    ```
    anchor test
    ```

If you encounter any questions or issues during the installation process or have any inquiries related to the task, please feel free to initiate a discussion on Discord within the Issues Forum.

### Hints and Useful Links
[Program Derived Address](https://solanacookbook.com/core-concepts/pdas.html)

[Account Context](https://docs.rs/anchor-lang/latest/anchor_lang/derive.Accounts.html)

[Account Model](https://solana.wiki/zh-cn/docs/account-model/)

[Solana Development Course](https://www.soldev.app/course)


-----

### Need help?
If you have any questions feel free to reach out to us at [Discord](https://discord.gg/z3JVuZyFnp).
