fn merge(l1: &Vec<i32>, s: usize, m: usize, e: usize, l2: &mut Vec<i32>) {
    let mut ptr1 = s;
    let mut ptr2 = m;
    for i in s..e {
        if (ptr1 < m) && (ptr2>=e || l1[ptr1] <= l1[ptr2]) {
            l2[i] = l1[ptr1];
            ptr1 += 1;
        }else{
            l2[i] = l1[ptr2];
            ptr2 += 1;
        }
    }
}

fn merge_copy(l1: &mut Vec<i32>, s: usize, e: usize, l2: &Vec<i32>) {
    (s..e).for_each(|i| l1[i] = l2[i]);
}

fn merge_split(l1: &mut Vec<i32>, s: usize, e:usize, l2: &mut Vec<i32>) {
    if e - s > 1{
        let m: usize = (s+e)/2;
        merge_split(l1, s, m, l2);
        merge_split(l1, m, e, l2);
        merge(l1, s, m, e, l2);
        merge_copy(l1, s, e, l2);
    }
}

fn sort(l: &mut Vec<i32>) {
    let size: usize = l.len();
    let mut worker: Vec<i32> = vec![0; size];
    merge_split(l, 0, size, &mut worker);
}

fn main(){
    let l1 = &mut vec![1,3,5,2,6,4];
    sort(l1);
    println!("after sort {:?}", l1);
}
