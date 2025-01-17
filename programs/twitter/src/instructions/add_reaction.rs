use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_reaction(ctx: Context<AddReactionContext>, reaction: ReactionType) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &mut ctx.accounts.tweet_reaction;

    // -------------------------------------------------------------------------------------------
    // TODO: The input parameter of this function is a reaction, which is of type enum ReactionType,
    // you can see that within lib.rs this enum helps us to clearly differentiate what reaction
    // we want to perform (Like/Dislike). Based on this fact you should check what type of
    // reaction we are getting and if the reaction is ReactionType::Like, you should
    // increment the number of likes within the tweet Account. If the reaction is of type
    // ReactionType::Dislike, you should increment the number of dislikes. Return an error in
    // case of over/underflow. Finally save the reaction value to the tweet_reaction variable of
    // the Reaction data account.

    // HINT1: try to use pattern matching via the match keyword
    // https://doc.rust-lang.org/rust-by-example/flow_control/match.html

    // HINT2: tweet.likes = tweet.likes.checked_add(1).ok_or(TwitterError::MaxLikesReached)?;
    // -------------------------------------------------------------------------------------------

    // TODO: Do not forget to update all values within the tweet_reaction Account

    // HINT:
    // - reaction_author
    // - parent_tweet
    // - bump (check initialize_tweet for reference)
    // -------------------------------------------------------------------------------------------

    Ok(())
}
#[derive(Accounts)]
pub struct AddReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
        init,
        payer = reaction_author,
        space = 8 + Reaction::LEN,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref(),
            ],
        bump)]
    pub tweet_reaction: Account<'info, Reaction>,
    // -------------------------------------------------------------------------------------------
    // TODO fill the required account macro below - we want to check that a PDA account with correct
    // seeds and bump was submitted

    // HINT:
    // - account must be mutable
    // - seeds are :    tweet.topic[..tweet.topic_length as usize].as_ref()
    //                  TWEET_SEED.as_bytes(),
    //                  tweet.tweet_author.key().as_ref()
    // - lastly, check the correctness of bump using: bump = tweet.bump
    // -------------------------------------------------------------------------------------------
    #[account()]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}
