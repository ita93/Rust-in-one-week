fn insert_sort(list: &mut [i32]) {
    for i in 1..list.len() {
        let key = list[i];
        let mut t:i32 = (i-1) as i32;
        let mut j = i-1;

        while list[j] > key{
            list[j+1] = list[j];
            if t < 0 {
                break;
            }
            if j!=0 {
                j = j - 1;
            }
            t = t-1;
        }
        j = (t+1) as usize;
        list[j] = key;
    }
}
fn main () {
    let my_list = &mut[5,3,4,2,1,8,10,234,22,22,55];
    insert_sort(my_list);
    println!("After sort: {:?}", my_list);
}
