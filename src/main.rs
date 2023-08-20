// fn get_first_element<T>(vec: Vec<T>) -> Option<T>{
//     vec.into_iter().next()
// }
// fn main(){
//     let vec1 = vec![1,2,3];
//     let vec2: Vec<i32>=vec![];
//     println!("{:?}",vec1);
//     let first_element1=get_first_element(vec1);
//     let first_element2=get_first_element(vec2);
//     println!("{:?}",first_element1);
//     match first_element1{
//         Some(element)=> println!("the first element is {}",element),
//         None=> println!("empty"),}
//         match first_element2{
//             Some(element)=> println!("the first element is {}",element),
//             None=> println!("empty"),}
// }
// use std::collections::HashMap;
// fn main(){
//     let vec=vec![1,2,3,4,5,6,7,8,9,10];

//     let iter=vec.iter();
//     let vec_even: Vec<_> = iter.filter(|x| **x > 5).map(|x| x * 2).collect();
//     println!("{:?}", vec_even);
//     let mut stock_prices = HashMap::new();
//     stock_prices.insert("TSLA", 20);
//     stock_prices.insert("MSFT", 40);
//     stock_prices.insert("APPL", 70);
//     for (stock, value) in stock_prices.iter() {
//         if *value < 50 {
//         println!("{}", stock);
//         }
//         }
// }

// struct Novel{
//     title: String,
//     author:String,
//     genre:String,
// }
// struct NonFiction {
//     title: String,
//     author: String,
//     topic: String,
// }
// trait Book{
//     fn get_summary(&self) -> ();
// }
// impl Book for Novel{
//     fn get_summary(&self)-> (){
//         println!("{} is a {} novel written by {}", self.title,self.genre,self.author);
//     }
// }
// impl Book for NonFiction{
//     fn get_summary(&self)-> (){
//         println!("{} is a  non-fiction book on {}  written by {}", self.title,self.topic,self.author);
//     }
// }
// fn main(){
//     let book_1=Novel{
//         title: String::from("Harry Potter"),
//         author: String::from("JK ROWLING"),
//         genre: String::from("Wizard"),
//     };

//     let book_2=NonFiction{
//         title: String::from("A Brief History of Time"),
//         author: String::from("Stephen Hawking"),
//         topic: String::from("the universe and its origins"),
//     };

//     book_1.get_summary();
//     book_2.get_summary();
// }
// fn is_this_cash(payment_type: String) -> Result<String, String>{
//     if payment_type=="cash".to_owned(){
//         Ok("Yes this is cash".to_owned())

//         }else{
//             Err("No this is not cash".to_owned())
        
//     }
// }

// fn main(){
//     let cash: Option<String>=Some("cash".to_owned());
//     let credit: Option<String>=Some("credit".to_owned());
//     if let Some(payment_type)=cash{
//         match is_this_cash(payment_type){
//             Ok(x)=>println!("output:{}",x),
//             Err(e)=>println!("output:{}",e)
//         }
//     }
//     if let Some(payment_type)=credit{
//         match is_this_cash(payment_type){
//             Ok(x)=>println!("output:{}",x),
//             Err(e)=>println!("output:{}",e)
//         }
//     }
// }
struct UserAccount {
    name: String,
    age: Option<u32>,
}
trait Balance {
    fn get_balance(&self) -> u32{
    10
    }
}
impl Balance for UserAccount {
}
fn increase_balance<T:Balance>(x: &T, amount:u32)-> Result<u32, String>{
    if amount <=10{
        return Ok(x.get_balance()+amount);
    }else {
        return Err("Increase must be less than 10!".to_owned());
    }
}
fn main(){
    let user_account = UserAccount{
        name:"self".to_owned(),
        age:Some(75),
    };
    match increase_balance(&user_account, 9) {
        Ok(v) => println!("UserAccount balance increased to {}", v),
        Err(e) => println!("{}", e),
}
    if let Some(age) = user_account.age {
    println!("UserAccount age is {}", age);
    }
}