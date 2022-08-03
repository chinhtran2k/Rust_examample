pub fn example_two() {
    let org_arr = vec![1,2,3,5,6,8,10,11];
    let sub_arr = vec![6,8,10];
    let sub_arr_new = &sub_arr;
    let mut arr = vec![];
    for idx1 in sub_arr_new {
        for idx2 in org_arr.iter().cloned(){
            if *idx1 == idx2 {
                arr.push(idx1);
            }
        }
    }
    if sub_arr_new.len() == arr.len(){
        println!("True")
    }
    else {
        println!("False")
    }
}