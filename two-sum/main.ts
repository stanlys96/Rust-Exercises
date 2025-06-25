const twoSum = (numArray: number[], target: number) => {
  for (let i = 0; i < numArray.length; i++) {
    for (let j = i + 1; j < numArray.length; j++) {
      if (numArray[i] + numArray[j] === target) {
        return [i, j];
      }
    }
  }
  return [];
};

console.log(twoSum([2, 3, 4, 5], 9));
console.log(twoSum([2, 7, 11, 15], 9));
console.log(twoSum([3, 2, 4], 6));
console.log(twoSum([3, 3], 6));
