##CODE_HERE##

pub fn main() {
    let mut input = String::new();
io::stdin().read_to_string(&mut input).unwrap();

let mut it = input.split_whitespace();
let size_arr0: usize = it.next().unwrap().parse().unwrap();

let arr:Vec<i32> = Vec::new();
for i in 0..size_arr0 {
    arr.push(it.next().unwrap().parse::<i32>().unwrap());
}
let target = it.next().unwrap().parse::<i32>().unwrap();
let result: i32 ;
result = twoSum(arr, target);
println!("{}", result);

}
