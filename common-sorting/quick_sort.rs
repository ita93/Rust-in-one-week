fn quick_sort(list: &mut Vec<i32>, left: usize, right: usize){
    let (mut i, mut j) = (left, right);
    let pivot = list[(left+right)/2];
    while i <= j {
        while list[i] < pivot {
            i = i + 1;
        }
        while list[j] > pivot {
            j = j - 1;
        }
        if i <= j {
            list.swap(i, j);
            i = i+1;
            j = j-1;
        }
    }

    if left < j {
        quick_sort(list, left, j);
    }
    if i < right {
        quick_sort(list, i, right);
    }
}

fn main() {
    let my_vec = &mut vec![5,3,4,1,2];
    let size = my_vec.len() - 1;
    quick_sort(my_vec, 0, size);
    println!("Hello world!! {:?}", my_vec);
}
