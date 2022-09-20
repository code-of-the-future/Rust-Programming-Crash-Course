// Structs #2

// Creating a struct with functions

struct Education {
    college_name: String,
    major_title: String,
    minor_title: String,
}

// The impl keyword is primarily used to define
// implementations on types. Implementation blocks
// will house the functions and methods associated
// with our struct. 

impl Education {
    // Construct the education
    // Associative Function
    fn constructing_education(
        college: &str,
        major: &str,
        minor: &str,
        ) -> Education {
        Education {
            college_name: college.to_string(),
            major_title: major.to_string(),
            minor_title: minor.to_string(),
        }
    }

    // Methods - the first argument is always &self which
    // is the instance the method is being called on.
    // Associative functions don't need this 
    fn full_education(&self) -> String {
        format!("{}, {}, {}", 
            self.college_name,
            self.major_title,
            self.minor_title)
    }

    // Change major
    fn change_major(&mut self, major: &str){
        self.major_title = major.to_string()
    }

}


fn main(){
    let mut education1 = Education::constructing_education("Cambridge", "Maths", "Astrophysics");
    println!("College: {}. Major: {}. Minor: {}.", 
        education1.college_name,
        education1.major_title,
        education1.minor_title);

    println!("Education: {}", education1.full_education());

    // Change the major:
    education1.change_major("Computer Science");
    println!("Education: {}", education1.full_education()); 


}