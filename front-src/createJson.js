const fs = require("fs");

const MIN = 0, MAX =Math.pow(2, 10) - 1;
const FILENAME = "randoms.json";

const createMaxIntArray = () => {
  const result = [];
  for (let i = MIN; i < MAX; i++) {
    result.push(i);
  }
  return result;
}

const shuffleArray = (array) => {
  let tmp = 0;
  let j = 0;
  for (let i = array.length - 1; i > 0; i--) {
    j = Math.floor(Math.random() * (i + 1));
    tmp = array[i];
    array[i] = array[j];
    array[j] = tmp;
  }
  console.log(array);
  return array;
}

const createJSON = randoms => {
    fs.writeFile(FILENAME, JSON.stringify(randoms),
    (err) => { console.log(err) })
};

createJSON(shuffleArray(createMaxIntArray()));
