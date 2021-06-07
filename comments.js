const generate = () => {
  let d = new Date();
  const timestamp = new Date(
    d.getUTCFullYear(),
    d.getUTCMonth(),
    d.getUTCDate(),
    d.getUTCHours(),
    d.getUTCMinutes(),
    d.getUTCSeconds()
  );
  return timestamp;
};

const convert = (date) => {
  const oldDate = new Date(date);
  const currentDate = new Date(generate());

  let ms = Math.abs(currentDate - oldDate);
  let prettyTime;

  if (ms >= 1000 * 60 * 60 * 24 * 365) {
    prettyTime = Math.floor(ms / (1000 * 60 * 60 * 24 * 365)) + ' years';
  } else if (ms >= 1000 * 60 * 60 * 24 * 30) {
    prettyTime = Math.floor(ms / (1000 * 60 * 60 * 24 * 30)) + ' months';
  } else if (ms >= 1000 * 60 * 60 * 24) {
    prettyTime = Math.floor(ms / (1000 * 60 * 60 * 24)) + ' days';
  } else if (ms >= 1000 * 60 * 60) {
    prettyTime = Math.floor(ms / (1000 * 60 * 60)) + ' hours';
  } else if (ms >= 1000 * 60) {
    prettyTime = Math.floor(ms / (1000 * 60)) + ' minutes';
  } else {
    prettyTime = Math.floor(ms / 1000) + ' seconds';
  }
  // Remove pluralization if needed and return
  return (
    (prettyTime.split(' ')[0] === '1' ? prettyTime.slice(0, -1) : prettyTime) +
    ' ago'
  );

  // return new Date(
  //   Date.UTC(
  //     oldDate.getFullYear(),
  //     oldDate.getMonth(),
  //     oldDate.getDate(),
  //     oldDate.getHours(),
  //     oldDate.getMinutes(),
  //     oldDate.getSeconds()
  //   )
  // );
};

const comments = [
  {
    commentID: '0',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 0',
    reply_to: '',
  },
  {
    commentID: '1',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 1',
    reply_to: '0',
  },
  {
    commentID: '2',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 2',
    reply_to: '1',
  },
  {
    commentID: '3',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 3',
    reply_to: '0',
  },
  {
    commentID: '4',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 4',
    reply_to: '',
  },
  {
    commentID: '8',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 8',
    reply_to: '7',
  },
  {
    commentID: '5',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 5',
    reply_to: '',
  },
  {
    commentID: '6',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 6',
    reply_to: '5',
  },
  {
    commentID: '7',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 7',
    reply_to: '2',
  },
  {
    commentID: '9',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 9',
    reply_to: '5',
  },
  {
    commentID: '10',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 10',
    reply_to: '6',
  },
  {
    commentID: '11',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 11',
    reply_to: '3',
  },
  {
    commentID: '12',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 12',
    reply_to: '3',
  },
  {
    commentID: '13',
    posted_on: generate(),
    modified_on: generate(),
    reacts: [],
    comment: 'Test Comment 13',
    reply_to: '11',
  },
];

export default comments;
