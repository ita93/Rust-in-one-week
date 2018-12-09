fn bubble_sort(list: &mut[i32]) {
    for i in 0..list.len()-1 {
        for j in 0..(list.len() - i - 1) {
            if list[j] > list[j+1] {
                list.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let my_list = &mut[5,3,4,1,2];
    bubble_sort(my_list);
    println!("After sort: {:?}", my_list);
}
