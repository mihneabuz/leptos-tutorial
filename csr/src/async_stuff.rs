use gloo_timers::future::TimeoutFuture;
use leptos::{html::Input, *};

async fn load_data(secs: u32) -> String {
    TimeoutFuture::new(secs * 1000).await;
    String::from("Hello world!")
}

#[component]
fn LoadData() -> impl IntoView {
    let once = create_resource(|| (), |_| async move { load_data(1).await });

    view! {
      <h4>"My Data"</h4>
      {move || match once.get() {
          None => view! { <p>"Loading..."</p> }.into_view(),
          Some(data) => view! { <p>{data}</p> }.into_view(),
      }}

      <button on:click=move |_| once.refetch()>Refresh</button>
    }
}

#[component]
fn Suspension() -> impl IntoView {
    let a = create_resource(|| (), |_| async move { load_data(1).await });
    let b = create_resource(|| (), |_| async move { load_data(3).await });

    // Can also use Transition - handles refresh better
    view! {
      <h4>"My Data"</h4>
      <Suspense fallback=move || view! { <p>"Loading..."</p> }>
        <h5>"A"</h5>
        {move || { a.get().map(|a| view! { <div>{a}</div> }) }}

        <h5>"B"</h5>
        {move || { b.get().map(|b| view! { <div>{b}</div> }) }}

      </Suspense>
    }
}

#[component]
fn SimpleAwait() -> impl IntoView {
    async fn fetch_monkeys() -> i32 {
        load_data(2).await;
        3
    }

    view! {
      <Await future=fetch_monkeys let:data>
        <p>{*data} " little monkeys, jumping on the bed."</p>
      </Await>
    }
}

#[component]
fn Action() -> impl IntoView {
    let add_todo = create_action(|_: &String| async move { load_data(3).await });

    let submitted = add_todo.input();
    let pending = add_todo.pending();
    let todo_id = add_todo.value();

    let input_ref = create_node_ref::<Input>();

    view! {
      <form on:submit=move |ev| {
          ev.prevent_default();
          let input = input_ref.get().expect("input to exist");
          add_todo.dispatch(input.value());
      }>

        <label>"What do you need to do?" <input type="text" node_ref=input_ref/></label>
        <button type="submit">"Add Todo"</button>
      </form>
      <p>{move || pending().then_some("Loading...")}</p>
      <p>"Submitted: " <code>{move || format!("{:#?}", submitted())}</code></p>
      <p>"Pending: " <code>{move || format!("{:#?}", pending())}</code></p>
      <p>"Todo ID: " <code>{move || format!("{:#?}", todo_id())}</code></p>
    }
}

#[component]
pub fn AsyncStuff() -> impl IntoView {
    view! {
      <SimpleAwait/>
      <LoadData/>
      <Suspension/>
      <Action/>
    }
}
