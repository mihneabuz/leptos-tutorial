use leptos::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <div>
                <p>hello world !</p>
                <App/>
                <Weird/>
            </div>
        }
    })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let increase = move |_| set_count.update(|old| *old += 1);
    let reset = move |_| set_count(0);

    let is_odd = move || count() % 2 == 1;

    view! {
        <button on:click=increase>"Increase"</button>

        <span class:red=is_odd style:margin="1em">
            {count}
        </span>

        <button on:click=reset>"Reset"</button>

        <progress style:display="block" max="20" value=count />
    }
}

#[component]
fn Weird() -> impl IntoView {
    let (x, set_x) = create_signal(0);

    view! {
        <button
            on:click=move |_| set_x.update(|n| *n += 20)
            style:position="absolute"
            style:left=move || format!("{}px", x() + 200)
            style:top=move || format!("{}px", x() + 200)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            style=("--columns", x)
        >
            "Click to Move"
        </button>
    }
}
