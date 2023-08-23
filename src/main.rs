fn main(){
    //________
    // VAriables
    //_________

    let x=15;
    println!("The value of the variable x={}",x);
    let mut c=60;
    println!("{}",c);
    c=17;
    println!("{}",c);

    let y=5*4;
    println!("value of 5*4={}",y);
    //_________
    //   -Data Types
    //     -Scalar Data Types
    // int
    //       -signed     i8,i16,i32,i64   
    //        -unsigned   u8,u16,u32,u64

    println!("The maximum number in i8 is equal to {}",std::i8::MAX);
    println!("The minimum number in i16 is qual to {}",std::i16::MIN);
    println!("The minimum number in u16 is qual to {}",std::u16::MIN);
    //_______
    //  Floats 
    //     f32,f64
    //___________
    let z=3.64;
    println!("The maximum number in f32 is {}",std::f32::MAX);
    //____________
    //  Boolean
    //____________
    let status:bool=false;
    println!("The values of some of our variables are {:?}",(x,y,z,status));
    let not_equals=18!=18;
    println!("The value of condition 18!=18 is {}",not_equals);
    //____________
    //   characters
    //_______________

    let c1='a';
    let c2='+';
    let c3='3';
    let c4='\u{288A}';
    let c5='\"';


    println!("The characters are {:?}",(c1,c2,c3,c4,c5));
}






