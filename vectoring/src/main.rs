#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);

    let third: &i32 = &v[0];
//    let third: Option<&i32> = v.get(0);
 //   v.push(9);
    println!("{}", third);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
    println!("After adding 50");
    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("Blue")),
        SpreadSheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }

}
