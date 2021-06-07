use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct Comment {
    comment_id: String,
    comment: String,
    reply_to: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Comments {
    comments: Vec<Comment>,
}

pub fn parser() -> Result<()> {
    let data = r#"
{
    "comments": [
        {
            "comment_id": "0",
            "comment": "Test Comment 0",
            "reply_to": ""
        },
        {
            "comment_id": "1",
            "comment": "Test Comment 1",
            "reply_to": "0"
        },
        {
            "comment_id": "2",
            "comment": "Test Comment 2",
            "reply_to": "1"
        },
        {
            "comment_id": "3",
            "comment": "Test Comment 3",
            "reply_to": "0"
        },
        {
            "comment_id": "4",
            "comment": "Test Comment 4",
            "reply_to": ""
        },
        {
            "comment_id": "8",
            "comment": "Test Comment 8",
            "reply_to": "7"
        },
        {
            "comment_id": "5",
            "comment": "Test Comment 5",
            "reply_to": ""
        },
        {
            "comment_id": "6",
            "comment": "Test Comment 6",
            "reply_to": "5"
        },
        {
            "comment_id": "7",
            "comment": "Test Comment 7",
            "reply_to": "2"
        },
        {
            "comment_id": "9",
            "comment": "Test Comment 9",
            "reply_to": "5"
        },
        {
            "comment_id": "10",
            "comment": "Test Comment 10",
            "reply_to": "6"
        },
        {
            "comment_id": "11",
            "comment": "Test Comment 11",
            "reply_to": "3"
        },
        {
            "comment_id": "12",
            "comment": "Test Comment 12",
            "reply_to": "3"
        },
        {
            "comment_id": "13",
            "comment": "Test Comment 13",
            "reply_to": "11"
        }
    ]
}"#;

    let c: Comments = serde_json::from_str(data)?;

    println!("{:#?}", c);

    Ok(())
}
