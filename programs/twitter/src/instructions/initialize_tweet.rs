use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn initialize_tweet(
    ctx: Context<InitializeTweet>,
    topic: String,
    content: String,
) -> Result<()> {
    let initialized_tweet = &mut ctx.accounts.tweet;

    require!(
        topic.as_bytes().len() <= TOPIC_LENGTH,
        TwitterError::TopicTooLong
    );

    // -------------------------------------------------------------------------------------------
    // TODO make sure to also check for the length of Content

    // HINT:    You can use error : TwitterError::ContentTooLong
    //          You should use constant CONTENT_LENGTH
    // -------------------------------------------------------------------------------------------

    // NOTICE how we copy data from String into bytearray
    // firstly we create empty bytearray of predefined length (depends on the String we want to
    // save inside)
    let mut topic_data = [0u8; TOPIC_LENGTH];
    // then we copy contents of the String into the bytearray
    topic_data[..topic.as_bytes().len()].copy_from_slice(topic.as_bytes());
    // lastly we assign the bytearray into the bytearray stored within the Tweet Account
    initialized_tweet.topic = topic_data;

    // Same steps as above but now for content string
    let mut content_data = [0u8; CONTENT_LENGTH];
    content_data[..content.as_bytes().len()].copy_from_slice(content.as_bytes());
    initialized_tweet.content = content_data;

    // -------------------------------------------------------------------------------------------
    // TODO: Do not forget to update also other fields within the Tweet account

    // HINT:
    // - topic_length
    // - tweet_author
    // - likes
    // - dislikes
    // - bump is already updated below
    // -------------------------------------------------------------------------------------------
    initialized_tweet.bump = ctx.bumps.tweet;

    Ok(())
}

#[derive(Accounts)]
#[instruction(topic: String)]
pub struct InitializeTweet<'info> {
    #[account(mut)]
    pub tweet_authority: Signer<'info>,
    #[account(
        init,
        payer = tweet_authority,
        space = 8 + Tweet::LEN,
        seeds = [
            topic.as_bytes(),
            TWEET_SEED.as_bytes(),
            tweet_authority.key().as_ref()
            ],
        bump)]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}
