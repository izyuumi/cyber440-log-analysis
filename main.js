// read local file

const fs = require("fs");

fs.readFile("Elk Logs", "utf8", (err, data) => {
  if (err) {
    console.error(err);
    return;
  }
  const logs = JSON.parse(data);

  for (const log of logs.aggregations[2].buckets) {
    console.log(log.key_as_string.substring(5, 16));
    // console.log(log.doc_count);
  }
});
