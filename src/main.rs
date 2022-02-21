
fn main() {
    // println!("Hello, world!");
    two_sums(&[2,3,4,5,6,7],9);
}
fn two_sums(test_array:  &[i64],num :i64) -> (){
    
        for i in IntoIterator::into_iter(test_array){
             for n in test_array{
                if i+n==num {
                  println!("{:?}",[*i,*n]);
                    
                }else{
                    println!("nothing adds up")
                }
            }
          
        }
      
    }
   

