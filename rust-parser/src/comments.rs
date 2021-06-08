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

#[derive(Debug, Clone)]
struct FirstStepComment {
    comment_id: String,
    comment: String,
    reply_to: String,
    initial_replies: Vec<String>
}

#[derive(Debug, Clone)]
struct FirstStepComments {
    comments: Vec<FirstStepComment>,
}

impl FirstStepComments {
    fn find_comment(&self, comment_id: String) -> Option<FirstStepComment> {
        for comment in self.comments.iter() {
            if comment.comment_id == comment_id {
                return Some(comment.clone());
            }
        }
        
        None
    }

    fn get_replies(c: Comments) -> FirstStepComments {
        let mut final_comments = Vec::<FirstStepComment>::new();

        for comment in c.comments.iter() {
            let mut replies = Vec::<String>::new();
            for reply in c.comments.iter() {
                if reply.reply_to == comment.comment_id {
                    replies.push(reply.comment_id.clone())
                }
            }

            final_comments.push(FirstStepComment {
                comment_id: comment.comment_id.clone(),
                comment: comment.comment.clone(),
                reply_to: comment.reply_to.clone(),
                initial_replies: replies
            });
        }

        FirstStepComments {
            comments: final_comments
        }
    }

    fn get_depth(&self) -> u32 {
        let mut depth = 0;
        let mut unique_ids = Vec::<String>::new();

        for comment in self.comments.iter() {
            let mut current_depth = 0;
            let mut current_comment = comment.clone();
            unique_ids.push(comment.comment_id.clone());

            while !current_comment.reply_to.is_empty() {
                current_depth += 1;
                match &self.find_comment(current_comment.reply_to) {
                    Some(c) => current_comment = c.clone(),
                    None => break,
                }
            }

            if current_depth > depth {
                depth = current_depth;
            }
        }

        depth
    }
}

pub fn parser(data: &str) -> Result<()> {
    let c: Comments = serde_json::from_str(data)?;

    let final_comments = FirstStepComments::get_replies(c);
    let depth = final_comments.get_depth();

    println!("{:#?}", final_comments);
    println!("\nDepth: {}", depth);

    Ok(())
}
