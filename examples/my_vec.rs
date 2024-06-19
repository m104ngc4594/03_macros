use anyhow::Result;

// my_vec! = my_vec! { 1, 2, 3 }; // Vec<i32>
#[macro_export]
macro_rules! my_vec {
    () => { Vec::new() };
    ($elem:expr; $n:expr) => { std::vec::from_elem($elem, $n) };
    ($($x:expr),+ $(,)?) => {
        {
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
    ];
    println!("{:?}", v);

    let v1 = <[_]>::into_vec(Box::new([1, 2, 3, 4]));
    println!("{:?}", v1);

    Ok(())
}
