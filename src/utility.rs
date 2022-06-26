use rand::Rng;



pub fn pi() -> f64{
    3.1415926535897932385
}

pub fn degrees_to_radians(degrees: f64) -> f64{
    degrees * pi() / 180.0
}

pub fn randomf64() -> f64{
    let mut rng = rand::thread_rng();
    let out : f64 = rng.gen();
    out
}

pub fn randomf64range(min: f64, max: f64) -> f64{
    let mut rng = rand::thread_rng();
    let out : f64 = rng.gen();
    (out * (max - min)) + min
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64{
    let mut a = x;
    if x < min{
        a = min;
    }
    if x > max{
        a = max;
    }
    a
}