##CODE_HERE##

let input = fs.readFileSync('/dev/stdin', 'utf8').trim().split('\n').join(' ').split(' ');
let size_arr0 = parseInt(input.shift());
let arr = input.splice(0, size_arr0).map(num => parseInt(num));
let target = parseInt(input.shift());
let result;
result = twoSum(arr, target);
console.log(result);

