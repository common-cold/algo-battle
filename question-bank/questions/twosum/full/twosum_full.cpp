##CODE_HERE##

int main() {
    int size_arr0;
cin>>size_arr0;
vector<int> arr(size_arr0);
for (int i=0; i< size_arr0; i++) {
    cin>>arr[i];
}
int target;
cin>>target;
vector<int> result;
result = twoSum(arr, target);
cout<<result;


    return 0;
}
