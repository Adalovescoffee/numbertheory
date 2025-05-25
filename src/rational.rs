

use std::{fmt,cmp::Ordering, ops::{Add, Div, Mul, Neg, Sub}};
// a rational number is the set of (a,b) in Z*N^{*} such that gcd(a,b) = 1 
// the purpose of this is to properly define numbers, first with rational numbers, then their continued fractions decomposition, then real numbers :D 
// no jk i mean     



// gcd implementation! 
fn gcd(mut a:i64, mut b:i64)-> (i64,usize) {
    let mut steps : usize = 0 ; 
    while b!=0 {
        let temp = b ;
        b = a%b ;
        a = temp;
        steps = steps+1 
    }
    (a.abs(),steps)
}
#[derive(Clone,Copy,Debug)]
pub struct Rational{
    numerator: i64, 
    denominator: i64,
    size:usize, //length of the gcd algo 


}

impl Rational{
    // as we know we should have the denominator to be different than 0 
   pub fn new(numerator:i64,denominator:i64)->Self{
        if denominator == 0 {
            panic!("division by 0!! wtf are you doing :/")

        }
        
        
            
        let (gcd,size) = gcd(numerator,denominator);
            
        let mut newnum = numerator/gcd;
        let mut newden = denominator/gcd;
        if newden < 0 {
            newden = - newden;
            newnum = -newnum;
            
        }
            
        Rational{numerator:newnum, denominator:newden,size}
            
        }

    
    pub fn size(&self) ->usize {
        self.size
    

        
    }
    pub fn continued_fraction(&self) -> (usize, [i64; 64]) {
        const MAX_STEPS: usize = 64;
        let mut quotients = [0i64; MAX_STEPS];
        let mut a = self.numerator;
        let mut b = self.denominator;
        let mut count = 0;

        while b != 0 && count < MAX_STEPS {
            quotients[count] = a / b;
            let r = a % b;
            a = b;
            b = r;
            count += 1;
        }

        (count, quotients)
    }
}
 


impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}
impl Neg for Rational {
    type Output = Rational; 
    fn neg(self)-> Rational{
        let numerator = -self.numerator;
        Rational::new(numerator,self.denominator)
    }

}
impl Add for Rational {
    type Output = Rational; 
    fn add(self, rat:Rational)-> Rational{
        let numerator = self.numerator*rat.denominator + self.denominator*rat.numerator;
        let denominator = self.denominator*rat.denominator;
        Rational::new(numerator,denominator)
    }
    

}
impl Sub for Rational {
    type Output = Rational; 
    fn sub(self, rat:Rational)-> Rational{
        let numerator = self.numerator*rat.denominator - self.denominator*rat.numerator; 
        let denominator= self.denominator*rat.denominator; 
        Rational::new(numerator,denominator)
    }
}
impl Mul for Rational {
    type Output = Rational; 
    fn mul(self, rat:Rational)->Rational{
        let numerator =self.numerator*rat.numerator;
        let denominator = self.denominator*rat.denominator; 
        Rational::new(numerator,denominator)

    }

}
impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.numerator * other.denominator).partial_cmp(&(other.numerator * self.denominator))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl PartialEq for Rational{
    fn eq(&self, other:&Rational)-> bool{
        self.numerator == other.numerator && self.denominator == other.denominator
        
    }
}
impl Eq for Rational{}
impl Div for Rational {
    type Output = Rational; 
    fn div(self, rat:Rational)->Rational{
        if rat.denominator == 0 {
            panic!("you really want to divide by 0?")
        }
        else {
            let numerator = self.numerator*rat.denominator; 
            let denominator = self.denominator*rat.numerator; 
            Rational::new(numerator, denominator)
        }
    }


}

//continued fraction implementation 
// since we're working on fractions for now (this will change) i can just have denom to be the size of my array 
//fn continuedfraction(r::Rational)->



fn main() {
    let a = Rational::new(2,6);
    let b = Rational::new(3,-39);
    println!("{}",a/b);

}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // this is a useless test and just to make sure that i know what i'm doing 
    fn test_regulardiv(){

        assert_eq!(3/4,0);
    }
    //regular gcd case
    #[test]
    fn test_gcdcase1(){

        assert_eq!(gcd(8,12).0,4);
    }
    //0 case 
    #[test]
    fn test_gcdcase2(){

        assert_eq!(gcd(8,0).0,8);
    }
    // negative version i know this works btw :p    
    #[test]
    fn test_gcdcase3(){

        assert_eq!(gcd(-60,36).0,12);
    }
    ////////////////////////////////////////////////////
    // 
    // 
    //now we're going to implement tests for the rational struct 
    // just checking i didn't do an oopsie ehehe 
    #[test]
    fn test_rationalreg(){
        let n = Rational::new(6,4);
        assert_eq!(n.numerator,3);
        assert_eq!(n.denominator,2);

    }
    // this one is for division by 0 
    #[test]
    #[should_panic]
    fn test_ratdivby0(){
        Rational::new(1,0);
    }
    // this one is for negative numbers  
    #[test]
    fn test_negativerat(){
        let m = Rational::new(8,-12);
        assert_eq!(m.numerator,-2);
        assert_eq!(m.denominator,3);

    }    
    ////////////////////////////////////////////////////////////////
    // now we're adding tests for operations 
    // first negation 
    #[test]
    
    fn test_negative(){
        let m = Rational::new(1,-12);
        assert_eq!(-m,Rational::new(1,12))
    }
    // now addition 
    #[test]
    fn test_addition(){
        let a = Rational::new(2,5);
        let b = Rational::new(-1,3);
        assert_eq!(a+b,Rational::new(1,15))
    }
    //substraction
    #[test]
    fn test_sub(){
        let a = Rational::new(2,5);
        let b = Rational::new(1,3);
        assert_eq!(a-b,Rational::new(1,15))
    }
    #[test]
    fn test_mul(){
        let a = Rational::new(2,5);
        let b = Rational::new(1,3);
        assert_eq!(a*b,Rational::new(2,15))
    }
    //division 
    #[test]
    fn test_div(){
        let a = Rational::new(2,5);
        let b = Rational::new(1,3);
        assert_eq!(a/b,Rational::new(6,5))

    }
    #[test]
    #[should_panic]
    fn test_div0(){
        let a = Rational::new(2,5);
        let b = Rational::new(0,3);
        let _m =  a/b;
        
    }
}

