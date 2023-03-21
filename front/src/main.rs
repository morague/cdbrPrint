mod app;
mod app_views;
mod generator;
mod job;

fn main() {
    yew::Renderer::<app::App>::new().render();
}

