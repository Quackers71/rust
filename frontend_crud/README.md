# Step-by-Step CRUD Application

### Step 1: Create a new Rust project

Create a new Rust project:
```
cargo new yew_crud_app
cd yew_crud_app
```

Update your Cargo.toml to include Yew and other dependencies:
```
[dependencies]
yew = { version = "0.19", features = ["csr"] }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
```
### Step 2: Create the Application Structure</br>
Here’s a simple structure of the application in your src/main.rs:
```
use yew::prelude::*;

struct Item {
    id: usize,
    name: String,
}

struct Model {
    items: Vec<Item>,
    input_value: String,
}

enum Msg {
    AddItem,
    UpdateInput(String),
    DeleteItem(usize),
}

impl Model {
    fn add_item(&mut self) {
        if !self.input_value.is_empty() {
            let id = self.items.len();
            self.items.push(Item {
                id,
                name: self.input_value.clone(),
            });
            self.input_value.clear();
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            items: Vec::new(),
            input_value: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddItem => {
                self.add_item();
                true
            }
            Msg::UpdateInput(value) => {
                self.input_value = value;
                true
            }
            Msg::DeleteItem(index) => {
                self.items.retain(|item| item.id != index);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let input_value = self.input_value.clone();
        let oninput = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateInput(input.value())
        });

        let add_item = {
            let link = ctx.link().clone();
            move |_| link.send_message(Msg::AddItem)
        };

        html! {
            <div>
                <input {oninput} value={input_value} placeholder="Enter an item"/>
                <button onclick={add_item}>{ "Add Item" }</button>
                <ul>
                    { for self.items.iter().map(|item| {
                        let index = item.id;
                        html! {
                            <li>
                                { &item.name }
                                <button onclick={ctx.link().callback(move |_| Msg::DeleteItem(index))}>{ "Delete" }</button>
                            </li>
                        }
                    }) }
                </ul>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
```

### Step 3: Build and Run

To build and run your application:

- 1. Create a new file called index.html in your project root:
```
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Yew CRUD App</title>
    <script type="module" src="yew_crud_app.js"></script>
</head>
<body>
    <div id="app"></div>
</body>
</html>
```

- 2.  Build and serve your application using Trunk:
```
trunk serve
```
This command starts a server at http://localhost:8080. You can then check the application in your web browser.

### Step 4: Modify and Expand

From this base application, you can expand CRUD capabilities:

Fetch and save items using an API: Use reqwest or another HTTP client for network requests to a REST API.
Integrate persistent storage: Look into storing the items in local storage or a database on the backend.
Extend UI: Use CSS frameworks like Tailwind CSS or Bootstrap for better styling.
Conclusion

This is a fundamental example of creating a CRUD application using Rust and Yew framework. Feel free to modify and extend this application as per your requirements! Happy coding!