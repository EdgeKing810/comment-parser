import fs from 'fs';
import comments from './comments.js';

let copiedComments = [...comments].sort((a, b) => new Date(a) - new Date(b));

copiedComments = copiedComments.map((c) => {
  let updatedComment = { ...c };
  const replies = copiedComments
    .filter((r) => r.reply_to === updatedComment.commentID)
    .map((r) => r.commentID);
  updatedComment.replies = [...replies];
  return updatedComment;
});

let maxLength = 0;
let uniqueComments = [];
for (let i = 0; i < copiedComments.length; i++) {
  let currentLength = 1;
  let currentComment = { ...copiedComments[i] };
  uniqueComments.push(currentComment.commentID);
  while (currentComment.reply_to) {
    currentComment = copiedComments.find(
      (c) => c.commentID === currentComment.reply_to
    );
    currentLength++;
  }

  maxLength = currentLength > maxLength ? currentLength : maxLength;
}

for (let count = 0; count < maxLength; count++) {
  let finalComments = [];

  for (let i = 0; i < copiedComments.length; i++) {
    let currentComment = { ...copiedComments[i] };
    let replies = [
      ...copiedComments[i].replies.filter((r) => typeof r === 'string'),
    ];
    let newReplies = copiedComments.filter(
      (c) =>
        replies.includes(c.commentID) &&
        (c.replies.length === 0 ||
          c.replies.every((r) => typeof r !== 'string'))
    );

    if (newReplies.length > 0) {
      let newIDs = newReplies.map((r) => r.commentID);
      let notFoundReplies = [...replies].filter((r) => !newIDs.includes(r));
      let diff = currentComment.replies.filter(
        (d) => typeof d !== 'string' && !newIDs.includes(d.commentID)
      );

      currentComment.replies = [...newReplies, ...notFoundReplies, ...diff];
    }
    finalComments.push(currentComment);
  }

  copiedComments = [...finalComments];
}

copiedComments = copiedComments.filter((c) => !c.reply_to);

let flattenedComments = [];
const getReplies = (comment, found, depth) => {
  const withoutReplies = { ...comment, depth: depth };
  delete withoutReplies.replies;
  found = [...found, withoutReplies];
  if (comment.replies.length > 0) {
    return comment.replies.map((r) => getReplies(r, found, depth + 1));
  } else {
    return [...found];
  }
};

for (let i = 0; i < copiedComments.length; i++) {
  const currentComments = getReplies(copiedComments[i], [], 0).flat(maxLength);
  const commentIDs = currentComments.map((c) => c.commentID);
  const cleanComments = currentComments.filter(
    (c, i) => commentIDs.indexOf(c.commentID) === i
  );
  flattenedComments.push([...cleanComments]);
}

flattenedComments = flattenedComments.flat(maxLength);

fs.writeFileSync('./comments.json', JSON.stringify(copiedComments));
fs.writeFileSync('./flatten.json', JSON.stringify(flattenedComments));
