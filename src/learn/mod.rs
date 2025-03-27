struct Car;
struct Truck;

enum VehicleType {
    Car(Car),
    Truck(Truck),
}

impl VehicleType {
    fn drive(&self) -> String {
        match self {
            VehicleType::Car(_) => "Car is driving".into(),
            VehicleType::Truck(_) => "Truck is driving".into(),
        }
    }
}

fn transform_in_car<'a>(veicle: &'a mut VehicleType) {
    *veicle = VehicleType::Car(Car);
}

#[test]
fn test_reference_mut() {
    let mut truck: VehicleType = VehicleType::Truck(Truck);
    println!("{:?}", truck.drive());

    transform_in_car(&mut truck);

    println!("{:?}", truck.drive());
}
