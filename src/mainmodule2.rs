#[derive(Debug,PartialEq)]
enum PaymentType {
    DigitalToken,
    Cash,
    }
    impl PaymentType{
        fn description(&self){
            match self{
                PaymentType::DigitalToken=>println!("Digital token"),
                PaymentType::Cash=>println!("Cash"),
            }
        }
    }
    #[derive(Debug)]
    struct Seller{
        payment_type: PaymentType,
        price:f32,
        balance:f32,
    }
    #[derive(Debug)]
    struct Buyer{
        name:String,
        payment_type:PaymentType,
        balance:f32,
    }
    #[derive(Debug)]
    struct BuyerGroup{
        members:Vec<Buyer>,
    }
    impl BuyerGroup{
        fn add_member(&mut self, h: Buyer) {
            self.members.push(h);
                }
    }
    impl BuyerGroup{
        fn find_buyer(&self,payment_type: &PaymentType) -> i32 {
            println!("searching for buyer with payment type {:?}",payment_type);
            let mut pos = 0;
            for i in &self.members{
                if i.payment_type==*payment_type{
                    println!("matching buyer using payment type {:?} was found at index{}",payment_type,pos);
                    println!("{:?}",i);
                    return pos;
                }
                pos +=1;
            }
            println!("buy with payment type {:?} not found",payment_type);
            return -1;
        }
        fn buy(&mut self,buyer_index: i32, seller:&mut Seller){
            let mut buyer = &mut self.members[buyer_index as usize];
            loop{
                if buyer.balance >= seller.price{
                    seller.balance += seller.price;
                    buyer.balance-=seller.price;
                    println!("{} balance: {}. seller balance :{}",buyer.name,buyer.balance,seller.balance);
                }else{
                    println!("{} balance {}insufficient. Seller balance:{}",buyer.name,buyer.balance,seller.balance);
                    break;
                }
            }
        }
    }
    
    
    fn main() {
        let buyer1 = Buyer {
            name: "John".to_owned(),
            payment_type: PaymentType::DigitalToken,
            balance: 100.00,
            };
            println!("{:?}",buyer1);
            let buyer2 = Buyer {
            name: "Sally".to_owned(),
            payment_type: PaymentType::Cash,
            balance: 100.00,
            };
            println!("{:?}",buyer2);
            let mut buyer_group =BuyerGroup{
                members: Vec::new(),
            };
            buyer_group.add_member(buyer1);
            buyer_group.add_member(buyer2);
            println!("{:?}",buyer_group);
            let mut seller = Seller {
                payment_type: PaymentType::DigitalToken,
                price: 10.0,
                balance: 0.0,
                };
                println!("{:?}",seller);
                let buyer_index = buyer_group.find_buyer(&seller.payment_type);
                if buyer_index >= 0{
                    buyer_group.buy(buyer_index,&mut seller);
                }

    }