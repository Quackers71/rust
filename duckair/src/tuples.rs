// Arrays and Tuples

pub fn run() {
    
    // Array
    // let location: [f64; 2] = [41.4094069, -81.8546911];

    // Tuple
    let location: (&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);
    println!("Location name: {}, Latitude: {}, Longitude: {}",
    location.0, location.1, location.2);

    // De-structuring the Array or Tuple
    let location: (&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);
    let (name, latitude, longitude) = location;
    println!("Location name: {}, Latitude: {}, Longitude: {}",
    name, latitude, longitude);
}