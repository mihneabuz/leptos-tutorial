use leptos::ev::SubmitEvent;
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
    let double_count = move || count() * 2;

    view! {
      <div>
        <button on:click=increase>"Increase"</button>
        <span class:red=is_odd style:margin="1em">
          {count}
        </span>
        <button on:click=reset>"Reset"</button>
      </div>

      <ProgressBarFn progress=double_count/>
      <ProgressBar progress=Signal::derive(double_count)/>

      <Show when=move || count() == 5>"Ding ding ding !!"</Show>

      <div style:height="100px"></div>

      <DynamicList initial_length=4/>

      <div style:height="100px"></div>

      <FormControlled/>

      <div style:height="100px"></div>

      <FormUncontrolled/>

      <div style:height="100px"></div>

      <NumericInput/>
    }
}

#[component]
fn ProgressBar(
    #[prop(default = 10)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress style:display="block" max=max value=progress></progress> }
}

#[component]
fn ProgressBarFn<F>(#[prop(default = 10)] max: u16, progress: F) -> impl IntoView
where
    F: Fn() -> i32 + 'static,
{
    view! { <progress style:display="block" max=max value=progress></progress> }
}

#[component]
fn Weird() -> impl IntoView {
    let (x, set_x) = create_signal(0);

    view! {
      <button
        on:click=move |_| set_x.update(|n| *n += 20)
        style:position="absolute"
        style:left=move || format!("{}px", x() + 300)
        style:top=move || format!("{}px", x() + 200)
        style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
        style:max-width="400px"
        style=("--columns", x)
      >
        "Click to Move"
      </button>
    }
}

#[component]
fn DynamicList(initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };

    let remove_counter = move |id| {
        set_counters.update(|counters| counters.retain(|(counter_id, _)| counter_id != &id))
    };

    view! {
      <div>
        <button on:click=add_counter>"Add Counter"</button>

        <ul>
          <For
            each=counters
            key=|counter| counter.0
            children=move |(id, (count, set_count))| {
                view! {
                  <li>
                    <button on:click=move |_| { set_count.update(|n| *n += 1) }>{count}</button>
                    <button on:click=move |_| remove_counter(id)>"Remove"</button>
                  </li>
                }
            }
          />

        </ul>
      </div>
    }
}

#[component]
fn FormControlled() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());
    let (value, set_value) = create_signal("B".to_string());

    view! {
      <input type="text" on:input=move |ev| set_name(event_target_value(&ev)) prop:value=name/>
      <p>Name is: {name}</p>

      <select on:change=move |ev| set_value(event_target_value(&ev))>
        <option value="A" selected=move || value() == "A">
          "A"
        </option>
        <option value="B" selected=move || value() == "B">
          "B"
        </option>
      </select>
    }
}

#[component]
fn FormUncontrolled() -> impl IntoView {
    let (name, set_name) = create_signal("Uncontrolled".to_string());

    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        set_name(input_element().unwrap().value());
    };

    view! {
      <form on:submit=on_submit>
        <input type="text" value=name node_ref=input_element/>
        <input type="submit" value="Submit"/>
      </form>

      <p>Name is: {name}</p>
    }
}

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    let on_input = move |ev| set_value(event_target_value(&ev).parse::<i32>());

    view! {
      <label>
        <span>"Type a number"</span>
        <input type="number" on:input=on_input/>
        <ErrorBoundary fallback=|errors| {
            let errors = move || {
                errors
                    .get()
                    .into_iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect_view()
            };
            view! {
              <div class="error">
                <p>"Not a number! Errors: "</p>
                <ul>{errors}</ul>
              </div>
            }
        }>
          <p>"You entered " <strong>{value}</strong></p>
        </ErrorBoundary>
      </label>
    }
}
