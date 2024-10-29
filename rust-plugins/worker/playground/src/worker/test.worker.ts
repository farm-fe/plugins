import { get } from "lodash-es"
self.onmessage = (e) => {
  console.log(get({ a: 500 }, "a"));
  console.log("Message received from main script");
  const workerResult = `Result: ${e.data[0] * e.data[1]}`;
  console.log("Posting message back to main script");
  console.log("worker result:", workerResult);
  postMessage(workerResult);
};
