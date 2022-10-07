use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::collections::HashMap;

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
    comment_id_index_map: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
struct FinalComment {
    comment_id: String,
    comment: String,
    depth: u32,
}

#[derive(Debug, Clone)]
struct FinalComments {
    comments: Vec<FinalComment>,
}

impl FirstStepComments {
    fn find_comment(&self, comment_id: String) -> Option<FirstStepComment> {
        match self.comment_id_index_map.get(&comment_id) {
            Some(index) => Some(self.comments[*index].clone()),
            None => None,
        }
    }

    fn get_replies(c: Comments) -> FirstStepComments {
        let mut final_comments = Vec::<FirstStepComment>::new();
        let mut comment_id_index_map = HashMap::<String, usize>::new();

        for (index, comment) in c.comments.iter().enumerate() {
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
            comment_id_index_map.insert(comment.comment_id.clone(), index);
        }

        FirstStepComments {
            comments: final_comments,
            comment_id_index_map,
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
        let mut comment_id_index_map = self.comment_id_index_map.clone();

        for _n in 0..depth {
            let mut final_comments = Vec::<FirstStepComment>::new();
            let mut final_comment_id_index_map = HashMap::<String, usize>::new();

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
                final_comment_id_index_map.insert(current_comment.comment_id.clone(), i);
            }

            copied_comments = final_comments;
            comment_id_index_map = final_comment_id_index_map;
        }

        FirstStepComments {
            comments: copied_comments
                .into_iter()
                .filter(|c| c.reply_to.is_empty())
                .collect::<Vec<FirstStepComment>>(),
            comment_id_index_map,
        }
    }
}

fn find_replies(
    comment: &FirstStepComment,
    res: Vec<FinalComment>,
    depth: u32,
) -> Vec<FinalComment> {
    let without_replies = FinalComment {
        comment_id: comment.clone().comment_id,
        comment: comment.clone().comment,
        depth,
    };

    let mut result = res.clone();
    result.push(without_replies);

    if !comment.clone().final_replies.is_empty() {
        for reply in comment.clone().final_replies.iter() {
            let mut tmp_res = find_replies(reply, result.clone(), depth + 1);
            for r in tmp_res.iter_mut() {
                result.push(r.clone());
            }
        }
        return result;
    } else {
        return result;
    }
}

impl FinalComments {
    fn construct(c: FirstStepComments) -> FinalComments {
        let mut all_comments: Vec<FinalComment> = vec![];

        for comment in c.comments.iter() {
            let mut all_replies = find_replies(comment, Vec::<FinalComment>::new(), 0);
            for reply in all_replies.iter_mut() {
                all_comments.push(reply.clone());
            }
        }

        FinalComments {
            comments: all_comments,
        }
    }

    fn sort(self, comment_ids: Vec<String>) -> FinalComments {
        let initial_comments = self.comments.clone();

        let size = comment_ids.len();
        let mut found_vector: Vec<bool> = Vec::with_capacity(size);
        for _ in 0..size {
            found_vector.push(false);
        }

        let mut final_comments: Vec<FinalComment> = vec![];
        for i in 0..initial_comments.len() {
            let index = comment_ids
                .iter()
                .position(|x| *x == initial_comments[i].comment_id)
                .unwrap();
            if !found_vector[index] {
                final_comments.push(initial_comments[i].clone());
                found_vector[index] = true;
            }
        }

        FinalComments {
            comments: final_comments,
        }
    }
}

pub fn parser(data: &str) -> Result<()> {
    let c: Comments = serde_json::from_str(data)?;

    let mut comment_ids: Vec<String> = vec![];
    for comm in c.comments.iter() {
        comment_ids.push(comm.comment_id.clone());
    }

    let mut comments = FirstStepComments::get_replies(c);
    let depth = comments.get_depth();
    comments = comments.construct_replies();

    println!("\nDepth: {}\n", depth);
    println!("Exploded:\n\n{:#?}", comments);

    let final_comments = FinalComments::construct(comments).sort(comment_ids);
    println!("\nFinal:\n\n{:#?}", final_comments);

    Ok(())
}
