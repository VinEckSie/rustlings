fn main() {
    // TODO: Fix the code to print "Hello world!".
    //println!("Hello world!");
   
   let mut counter = 0;

   let result = loop {
        counter += 1;

        let result2 = 'loop2: loop{
            if counter  == 2{
                break 'loop2 counter ;
            }

            counter += 1; 
        };

        println!("Inner loop returned: {}", result2);


        if counter == 5{
            break counter + 10;
        }
   };

   println!("result is {result}");

}
