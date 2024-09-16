
#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {Manual, SemiAuto, Automatic}

#[derive(PartialEq, Debug)]
enum Age {New, Used}

#[derive(PartialEq, Debug)]
enum Color {Blue, Green, Red, Silver}

#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {color: Color, motor: Transmission, roof: bool, age: (Age, u32)}

fn car_quality(miles: u32) -> (Age, u32) {
    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    let quality: (Age, u32) = if miles == 0 {
        (Age::New, 0)
    } else {
        (Age::Used, miles)
    };

    quality
}

fn car_factory(color: Color, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Use the values of the input arguments
    // All new cars always have zero mileage
    let car_age: (Age, u32) = car_quality(miles);
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_age,
    }
}


fn main() {
    // We haver orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    let mut engine: Transmission = Transmission::Manual;
    let mut car: Car = car_factory(Color::Blue, engine, false, 1500);
    println!("Car 1 = {:?}, {:?} transmission, convertible: {}, age: {:?}", car.color, car.motor, car.roof, car.age);
    engine = Transmission::Automatic;
    car = car_factory(Color::Green, engine, true, 0);
    println!("Car 2 = {:?}, {:?} transmission, convertible: {}, age: {:?}", car.color, car.motor, car.roof, car.age);
    engine = Transmission::SemiAuto;
    car = car_factory(Color::Silver, engine, false, 0);
    println!("Car 3 = {:?}, {:?} transmission, convertible: {}, age: {:?}", car.color, car.motor, car.roof, car.age);
    engine = Transmission::Manual;
    car = car_factory(Color::Red, engine, false, 0);
    println!("Car 4 = {:?}, {:?} transmission, convertible: {}, age: {:?}", car.color, car.motor, car.roof, car.age);

}
