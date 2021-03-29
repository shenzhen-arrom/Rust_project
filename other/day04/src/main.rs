fn main() {
    let light = TrafficLight::Red;
    notify(&light)
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
}

pub trait ReturnTimeForTrafficLight {
    
    fn changeStateTime(&self) ->i32;
    
}

impl ReturnTimeForTrafficLight for TrafficLight{

    fn changeStateTime(&self) ->i32{
         match &self {
            TrafficLight::Red =>  75,
            TrafficLight::Green =>  60,
            TrafficLight::Yellow =>  45,
             _ =>  30,
        }
    }
}
pub fn notify<T:ReturnTimeForTrafficLight> (item:&T){
    println!(" time:{}",item.changeStateTime())
}
