use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Comment {
    comment_id: String,
    comment: String,
    reply_to: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Comments {
    comments: Vec<Comment>,
}

#[derive(Debug)]
struct FinalComment {
    comment_id: String,
    comment: String,
    reply_to: String,
    initial_replies: Vec<String>
}

#[derive(Debug)]
struct FinalComments {
    comments: Vec<FinalComment>,
}

impl FinalComments {
    fn get_replies(c: Comments) -> FinalComments {
        let mut final_comments = Vec::<FinalComment>::new();

        for comment in c.comments.iter() {
            let mut replies = Vec::<String>::new();
            for reply in c.comments.iter() {
                if reply.reply_to == comment.comment_id {
                    replies.push(reply.comment_id.clone())
                }
            }

            final_comments.push(FinalComment {
                comment_id: comment.comment_id.clone(),
                comment: comment.comment.clone(),
                reply_to: comment.reply_to.clone(),
                initial_replies: replies
            });
        }

        FinalComments {
            comments: final_comments
        }
    }
}

pub fn parser(data: &str) -> Result<()> {
    let c: Comments = serde_json::from_str(data)?;

    let final_comments = FinalComments::get_replies(c);

    println!("{:#?}", final_comments);

    Ok(())
}
