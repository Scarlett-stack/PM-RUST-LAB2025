
//Define a structure Complex with floats.
use std::fmt; //pt display

#[derive(Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    //static method ca sa apelez Complex::new() -> fara paramaeru self!!
    fn new(real: f64, imaginary: f64) -> Complex {
        Complex {
            real,
            imaginary
        }
    }
    fn module(z: &Complex) -> f64 {
        return f64::sqrt(z.real * z.real + z.imaginary*z.imaginary);
    }

    fn multiply(z: &Complex, y: &Complex) -> Complex {
        Complex {
            real: z.real * y.real - z.imaginary*y.imaginary,
            imaginary: z.real * y.imaginary + z.imaginary * y.real,
        }
    }
}
// ca sa fol {} trb sa implemnt Display aici
impl fmt::Display for Complex {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        if self.imaginary > 0.0 {
            write!(f, "{} + {}i",self.real, self.imaginary)
        }
        else if self.imaginary <0.0 {
            write!(f, "{} {}i", self.real, self.imaginary)
        }
        else {
            write!(f,"{}", self.real)
        }
        //nu pun ; ca trb sa returneze write un result :) -> expresie nu statement
    }
}
fn main()
{
    let z = Complex::new(5.0, -6.0);
    println!("complex: {:#?}", z);

    let module: f64 = Complex::module(&z);
    println!("module: {}", module);

    let y = Complex::new(2.0,3.0);
    //let res =  z.multiply(&y); ok incerc cu ::
    let res = Complex::multiply(&z, &y);
    println!("multiplication is : {:#?}", res);

    // ce se intampla daca decomentez module ... ? ca fara el merge bine
    //ceva ciudat cu ownership
    //mda are sens daca fol doar referinte 

    println!("cu display: {}", z);
}