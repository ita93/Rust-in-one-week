fn selection_sort(list: &mut [i32]) {
    for i in 0..list.len() {
        let mut small = i;
        for j in (i+1)..list.len() {
            if list[j] < list[small] {
                small = j;
            }
        }
        list.swap(small, i);
    }
}
fn main() {
    let my_list = &mut[1,3,4,2,5,4,3,5,7,4,6];
    selection_sort(my_list);
    println!("Final result: {:?}", my_list);
}
