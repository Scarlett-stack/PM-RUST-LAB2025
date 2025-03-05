//lfa vibe check
use std::io::Write;
use std::io;
#[derive(Debug)]
struct MiniTuring {
    buff : [bool; 256],
    cursor: usize, //hehehehe
}

impl MiniTuring {
    fn new() -> MiniTuring {
        MiniTuring {
            buff: [false; 256],
            cursor: 0,
        }
    }
    //pot sa fac si display simplut fara fmt
    fn display(&self) {
        for b in self.buff {
            if b == true  
                {print!("1");}
            else {print!("0");}
        }
        println!();
    }
    /*
    Read the keyboard until "h" is received. 
    "l" will move the cursor to the left with wrap around, 
    "r" will move the cursor to the right with wrap around, 
    "1" will set the element at the cursor to true, 
    "0" will set the element at the cursor to false, 
    "p" prints the value at cursor, "h" displays the tape
     */
    fn compute(&mut self) { //trb mutable ref pt ca schimb chestii

        let mut band =  String::new();
       
        //nebunul asta face buffered print adica pana cand nu apare un \n nu printeaza...
        println!("input: ");
        // io::stdout().flush().unwrap(); -> util daca foloseai doar print 

        loop {
        //facem match cu break pe h si oprim loopul
        band.clear();
        //d-aia trb clear lol

        io::stdin().read_line(&mut band).expect("some sort of failure 1"); //asta face append...
        let current_char = band.trim(); //scot spatii si \n

       // print!("currentchar: {}",current_char);
        match current_char {
            "l" => self.cursor = (self.cursor + 255) % 256, //wrap around  0 1 .. 255 daca fac left pe 0 -> 255
            "r" => self.cursor = (self.cursor + 1) % 256,
            "1" => self.buff[self.cursor] = true,
            "0" => self.buff[self.cursor] = false,
            "p" => {println!("current elem: {}", self.buff[self.cursor]);},
            "h" => { self.display(); break;},
            _ => {println!("nu e char bun!!");},
        }
        }

    }

}
fn main() {
    let mut a = MiniTuring::new();
    //a.display();
    a.compute();
}