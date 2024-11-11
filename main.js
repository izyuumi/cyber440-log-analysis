// read local file

const fs = require("fs");

fs.readFile("Elk Logs", "utf8", (err, data) => {
  if (err) {
    console.error(err);
    return;
  }
  const logs = JSON.parse(data);

  let hits = logs.hits.hits;
  let start = 0;
  let success = 0;
  let failed = 0;

  for (let i = 0; i < hits.length; i++) {
    let message = hits[i]._source.message;
    if (message.includes("volume")) {
      let timestamp = hits[i]._source["@timestamp"];
      if (message.includes("started")) {
        start++;
        // console.log(`${timestamp},started`);
      }
      if (message.includes("successfully")) {
        success++;
        // console.log(`${timestamp},success`);
      }
      if (message.includes("failed")) {
        failed++;
        console.log(`${timestamp},failed`);
        console.log(`${message}`);
      }
    }
  }

  console.log(`Total Volume Started: ${start}`);
  console.log(`Total Volume Success: ${success}`);
  console.log(`Total Volume Failed: ${failed}`);
});
