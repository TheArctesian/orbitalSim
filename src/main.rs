use bevy::prelude::*;


fn test(){
    println!("we are working");
}

fn main() {
    App::new()
        .add_system(test)
        .run();
}
