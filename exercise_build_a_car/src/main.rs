// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    // Use the values of the input arguments
    // All new cars always have zero mileage
    Car { color: String::from(color), transmission, convertible, mileage: 0 }
}


fn main() {
    // We haver orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission,
              car.convertible, car.mileage);
    car = car_factory(String::from("Blue"), Transmission::Automatic, false);
    println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission,
              car.convertible, car.mileage);
              car = car_factory(String::from("Yellow"), Transmission::SemiAuto, true);
    println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission,
              car.convertible, car.mileage);
}
