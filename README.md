# Welcome to Hexgem

This project is mine attempt at creating game engine in rust. I am making this stuff with help of [Cherno`s tutorials](https://www.youtube.com/watch?v=JxIZbV_XjAs&list=PLlrATfBNZ98dC-V-N3m0Go4deliWHPFwT). If you want to share your knowledge or opinion on this project feel free to write me upon Discord _nick: mrsh33p_.

# Hexgem API

To start a window you need to implement HexgemApp trait. This will ensure that your struct can handle main features of this package. HexgemApp so far requires you to implement twwo more traits.

## Implementing App trait

This trait is where your app is suposed to come to life. You could write here all the event listeners and setup.

```rust
pub struct Sandbox {
    pub application: Application,
}

impl hexgem_engine::App for Sandbox {
    fn create_application() -> Self {
        let mut application = Application::new();
        return Sandbox {
            application: application,
        };
    }
}
```

## Implementing EventHandler trait

Here if you want you could overwrite handle_event or handle_category function, so you could catch events globaly. This is meant to be use when you want to perform some function every time the event happens.

```rust
impl hexgem_engine::EventHandler for Sandbox {}
```

## Implementing HexgemApp trait

This trait will give your struct run function that will execute the app. You just need to simply implement this to make it work.

```rust
impl hexgem_engine::HexgemApp for Sandbox {}
```

## Running app

Finally to run your app you need to add this to your main function. That lines will initialize logging system and create window.

```rust
fn main() {
    HexgemLogger::init().expect("Error occured on init logger");
    let sandbox = Sandbox::create_application();
    sandbox.run(&sandbox.application);
}
```

> If you are curious checkout [sandbox](https://github.com/MrSheep05/Hexgem/tree/main/hexgem_sandbox)
