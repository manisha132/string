fn main()
{
    let mut manu=String::from("manu");
    println!("Length:{}",manu.len());
    manu.push('s');
    manu.push_str("aroj");
    println!("{}",manu);
}