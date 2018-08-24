

struct Val {
    val: u32
}

trait Addd<T> {
    fn addd(self, toAdd: T) -> u32;
}

impl Addd<u32> for Val {
    fn addd(self, toAdd: u32) -> u32 {
        self.val + toAdd
    }
}

impl<'a> Addd<&'a String> for Val {
    fn addd(self, toAdd: &'a String) -> u32 {
        let a = toAdd.parse::<u32>().unwrap();
        self.val + a
    }
}

// struct GenVal<T>{
//     gen_val: T
// }

// // impl of Val
// impl Val {
//     fn value(&self) -> &f64 { &self.val }
// }

// // impl of GenVal for a generic type `T`
// impl <T> GenVal<T> {
//     fn value(&self) -> &T { &self.gen_val }
// }

fn main() {
    let x = Val { val: 3 };
    // let y = GenVal { gen_val: 3i32 };
    
    // println!("{}, {}", x.value(), y.value());
    // let s = "5";
    // let x = u32::from_str(s).unwrap();
    let s = String::from("4");
    println!("toAdd f64: {}", x.addd(&s));
    // println!("toAdd f64: {}", x.addd(4));
    
}
