use std::collections::HashMap;


#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {Manual, SemiAuto, Automatic}

#[derive(PartialEq, Debug)]
enum Age {New, Used}

#[derive(PartialEq, Debug)]
enum Color {Blue, Green, Red, Silver, Purple}

#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {color: Color, motor: Transmission, roof: bool, age: (Age, u32)}

// fn orders(order: u32, car: Car) -> HashMap<u32, Car> {
//     // Declare orders HashMap
//     let mut orders: HashMap<u32, Car> = HashMap::new();
//     orders.insert(order, car);
//     println!("{:?}", orders);
//     orders
// }

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

fn car_factory(color: Color, order: u32, miles: u32) -> Car {
    // Use the values of the input arguments
    // All new cars always have zero mileage
    // Add variety to orders for motor type and roof type
    let mut motor: Transmission = Transmission::Manual;
    let mut roof: bool = true;
    if order % 3 == 0 { // 3, 6 ,9 ...
        motor = Transmission::Automatic;
    } else if order % 2 == 0 { // 2, 4, 6 ...
        motor = Transmission::SemiAuto;
        roof = false;
    }
    Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    }
}

fn main() {
    let mut order: u32 = 1;
    let mut car: Car;
    let mut color: Color = Color::Blue;
    // Declare orders HashMap
    let mut orders: HashMap<u32, Car> = HashMap::new();
    // Start orders 
    car = car_factory(color, order, 1000);
    orders.insert(order, car);
    order = order + 1;
    color = Color::Green;
    car = car_factory(color, order, 2000);
    orders.insert(order, car);
    order = order + 1;
    color = Color::Red;
    car = car_factory(color, order, 2000);
    orders.insert(order, car);
    order = order + 1;
    color = Color::Silver;
    car = car_factory(color, order, 2000);
    orders.insert(order, car);
    order = order + 1;
    color = Color::Purple;
    car = car_factory(color, order, 2000);
    orders.insert(order, car);
    order = order + 1;
    color = Color::Green;
    car = car_factory(color, order, 2000);
    orders.insert(order, car);
    order = order + 1;
    color = Color::Green;
    car = car_factory(color, order, 2000);
    orders.insert(order, car);
    order = order + 1;
    color = Color::Green;
    car = car_factory(color, order, 2000);
    orders.insert(order, car);
    order = order + 1;
    color = Color::Green;
    car = car_factory(color, order, 2000);
    orders.insert(order, car);
    order = order + 1;
    color = Color::Green;
    car = car_factory(color, order, 2000);
    orders.insert(order, car);
    order = order + 1;
    color = Color::Green;
    car = car_factory(color, order, 2000);
    orders.insert(order, car);
    for (order, vehicle) in &orders {
        println!("{order:?} {vehicle:?}");
    }
}
