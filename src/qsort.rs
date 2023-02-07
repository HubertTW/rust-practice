
















/* 

// recursive quick sort
// time best case and average case O(nlogn)
// time worst case O(n^2)
// space O(logn)~O(n)

void quickSort(int[] arr,int low,int high){
    arr[low...high];


    if(low < high){

        int pivot = arr[low];
        int i = low;
        int j = high;

        while(i <= j){  // conquer(partition time) O(n)

             while(arr[j] >= arr[pivot]){
                 j--;
             }

             while(arr[i] < arr[pivot]){
                i++;
             }
            
                 swap(arr[i],arr[j]);
        }

         swap(arr[low],arr[j]);//立碑

         int mid = j;


    quickSort(arr,low,j-1); //divide O(logn) or O(n)
    quickSort(arr,j+1,high);



//recursive
int binarySearch(int arr[], int l, int r, int x)
{
if (r >= l) {
    int mid = (l + r - l) / 2;

    // If the element is present at the middle
    // itself
    if (arr[mid] == x)
        return mid;

    // If element is smaller than mid, then
    // it can only be present in left subarray
    if (arr[mid] > x)
        return binarySearch(arr, l, mid - 1, x);

    // Else the element can only be present
    // in right subarray
    return binarySearch(arr, mid + 1, r, x);
}

// We reach here when element is not
// present in array
return -1;
}


    }


}


//iteration
int binarySearch(int arr[], int l, int r, int x)
{
while (l <= r) {
    int m = l + (r - l) / 2;

    // Check if x is present at mid
    if (arr[m] == x)
        return m;

    // If x greater, ignore left half
    if (arr[m] < x)
        l = m + 1;

    // If x is smaller, ignore right half
    else
        r = m - 1;
}

// if we reach here, then element was
// not present
return -1;
}


//linear search 
int seqSearch(int arr[], int n, int x)
{
int i;
for (i = 0; i < n; i++)
    if (arr[i] == x)
        return i;
return -1;
}



void bubbleSort(int arr[], int n)
{
int i, j;
for (i = 0; i < n-1; i++)      

   // Last i elements are already in place   
   for (j = 0; j < n-i-1; j++) 
       if (arr[j] > arr[j+1])
          swap(&arr[j], &arr[j+1]);
}




void selectionSort(int arr[], int n)
{
int i, j, min_idx;

// One by one move boundary of unsorted subarray
for (i = 0; i < n-1; i++)
{
    // Find the minimum element in unsorted array
    min_idx = i;
    for (j = i+1; j < n; j++)
      if (arr[j] < arr[min_idx])
        min_idx = j;

    // Swap the found minimum element with the first element
    swap(&arr[min_idx], &arr[i]);
}
}



void insertionSort(int arr[], int n)
{
int i, key, j;
for (i = 1; i < n; i++) {
    key = arr[i]; //先store key值
    j = i - 1;

    /* Move elements of arr[0..i-1], that are
      greater than key, to one position ahead
      of their current position */
    while (j >= 0 && arr[j] > key) {
        arr[j + 1] = arr[j];
        j = j - 1;
    }
    arr[j + 1] = key;
}
}




















/* 


// recursive quick sort
// time best case and average case O(nlogn)
// time worst case O(n^2)
// space O(logn)~O(n)

void quickSort(int[] arr,int low,int high){
    arr[low...high];


    if(low < high){

        int pivot = arr[low];
        int i = low;
        int j = high;

        while(i <= j){  // conquer(partition time) O(n)

             while(arr[j] >= arr[pivot]){
                 j--;
             }

             while(arr[i] < arr[pivot]){
                i++;
             }
            
                 swap(arr[i],arr[j]);
        }

         swap(arr[low],arr[j]);//立碑

         int mid = j;


    quickSort(arr,low,j-1); //divide O(logn) or O(n)
    quickSort(arr,j+1,high);



*/



*/