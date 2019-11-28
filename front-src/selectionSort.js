const fs = require("fs");
const FILENAME = "randoms.json";
const { PerformanceObserver, performance } = require("perf_hooks");

const selectionSort = (array) => {
  let tmpSmallest,
    orderOfTmpSmallest,
    sortedOrder = [];
  for (let i = 0; i < array.length - 1; i++) {
    tmpSmallest = array[i];
    orderOfTmpSmallest = i;
    for (let j = i + 1; j < array.length; j++) {
      if (tmpSmallest > array[j]) {
        tmpSmallest = array[j];
        orderOfTmpSmallest = j;
      }
    }
    sortedOrder.push(orderOfTmpSmallest);
    array[orderOfTmpSmallest] = array[i];
    array[i] = tmpSmallest;
  }
  console.log(array);
  return array;
}

const readArray = () => {
  return JSON.parse(fs.readFileSync(FILENAME, 'utf8'));
}

const startTime = performance.now()
selectionSort(readArray());
const endTime = performance.now()
console.log(endTime - startTime);