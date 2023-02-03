pub fn adder(a: u32, b: u32) -> u32 {
   if a == 0 {
       return b;
   }
   if b == 0 {
       return a;
   }
   let carry = a^b;
   let sum = (a&b) << 1;
   return adder(sum, carry);

}
