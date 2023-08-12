
// struct User{
//     name:String,
//     balance:(f32,String),
//     }
// impl User{
//     fn print_name(&self){
//         println!("{}",self.name);
//     }
// }
// fn main() {
// let user=User{
//     name:"et".to_owned(),
//     balance:(99.99,"SGD".to_owned()),
// };
// user.print_name();
// println!("{},{}",user.balance.0,user.balance.1);
// }
// fn main(){
// let user_vec: Vec<String> = vec![
// "john".to_owned(),
// "mary".to_owned(),
// "simon".to_owned(),
// "john".to_owned(),
// "kelly".to_owned(),
// "harry".to_owned(),
// ];
// let mut counter = 0;
// // TODO: for loop to count number of john values
// for name in &user_vec { // loop every name in vector
//     if name == "john" { // if name name equals john
//     counter += 1; // increment counter by 1
//     }
//     }
// println!("{} johns in {:?}", counter, user_vec);
// }
struct User {
    name: String,
    balance: (f32, String)
    }
    impl User {
    fn print_user_detail(&self) {
    println!("Name: {}, Balance: {:?}",
    self.name, self.balance)
    }
    }

    fn main() {
        let mut user = User {
        name: "John".to_owned(),
        balance: (30000.0, "SGD".to_owned()),
        };
        let mut counter =0;
        for _ in 1..=10{
            counter +=1;
            if counter >10{
                break;
            }
            accrue_interest(&mut user, 3.0);
            println!("{}",counter);
        }
        
        }
        fn accrue_interest( user: &mut User, interest: f32) {
        user.balance.0 = user.balance.0 + (user.balance.0
        * interest / 100.0);
        user.print_user_detail();
        }
    
        