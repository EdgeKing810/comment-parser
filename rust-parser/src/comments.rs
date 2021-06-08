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
    initial_replies: Vec<String>,
    final_replies: Vec<FirstStepComment>,
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
                initial_replies: replies,
                final_replies: vec![],
            });
        }

        FirstStepComments {
            comments: final_comments,
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

    fn construct_replies(&mut self) -> FirstStepComments {
        let depth = self.get_depth();
        let mut copied_comments = self.comments.clone();

        for _n in 0..depth {
            let mut final_comments = Vec::<FirstStepComment>::new();

            for i in 0..(copied_comments.len()) {
                let current_comment = &copied_comments[i];
                let new_replies = &mut copied_comments[i].final_replies.clone();
                let initial_replies = &mut copied_comments[i].initial_replies.clone();

                for comm in copied_comments.iter() {
                    if comm.initial_replies.len() == 0
                        && current_comment
                            .initial_replies
                            .clone()
                            .iter()
                            .any(|r| *r == comm.comment_id)
                    {
                        let index = initial_replies
                            .iter()
                            .position(|x| *x == comm.comment_id)
                            .unwrap();
                        initial_replies.remove(index);
                        new_replies.push(comm.clone());
                    }
                }

                let new_comment = FirstStepComment {
                    comment_id: current_comment.comment_id.clone(),
                    comment: current_comment.comment.clone(),
                    reply_to: current_comment.reply_to.clone(),
                    initial_replies: initial_replies.to_vec(),
                    final_replies: new_replies.to_vec(),
                };

                final_comments.push(new_comment);
            }

            copied_comments = final_comments;
        }

        FirstStepComments {
            comments: copied_comments
                .into_iter()
                .filter(|c| c.reply_to.is_empty())
                .collect::<Vec<FirstStepComment>>(),
        }
    }
}

pub fn parser(data: &str) -> Result<()> {
    let c: Comments = serde_json::from_str(data)?;

    let final_comments = FirstStepComments::get_replies(c).construct_replies();
    let depth = final_comments.get_depth();

    println!("{:#?}", final_comments);
    println!("\nDepth: {}", depth);

    Ok(())
}
