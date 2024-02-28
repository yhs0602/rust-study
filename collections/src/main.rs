fn mean(v: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for i in v {
        sum = sum + i;
    }
    sum as f32 / v.len() as f32
}

fn sort(v: & mut [i32]) {
    // in place sort
    let len = v.len();
    if len < 2 {
        return;
    }
    let (front, back) = v.split_at_mut(len/2);
    sort(front);
    sort(back);
    // merge
    let mut newV = Vec::new();
    let mut mergexi = 0;
    let mut mergeyi = 0;
    loop {
        let v_front = front[mergexi];
        let v_back = back[mergeyi];
        if v_front > v_back {
            newV.push(v_back);
            mergeyi += 1;
        } else {
            newV.push(v_front);
            mergexi += 1;
        }
        if mergexi == front.len() {
            // flush back
            for i in mergeyi..back.len() {
                newV.push(back[i]);
            }
            break;
        }
        if mergeyi == back.len() {
            // flush front
            for i in mergexi..front.len() {
                newV.push(front[i]);
            }
            break;
        }
    }
    // write back to original v
    v.copy_from_slice(&newV);
}

fn median(v: &Vec<i32>) -> i32 {
   let mut new_vec = v.to_vec();
   sort(& mut new_vec);
   print!("Sorted:");
   for vv in &new_vec {
       print!("{},", vv);
   }
   println!(".");

   new_vec[new_vec.len()/2]
}

fn main() {
    println!("Hello, world!");
    let vv = vec![1, 2, 3, 19, 2, 3, 77, -11, 4, 0, -111];
    let sum = mean(&vv);
    println!("mean: {}", sum);
    let median = median(&vv);
    println!("median : {}", median);
}
