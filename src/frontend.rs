use shared_frontend::components::app_shell::AppShell;
use shared_frontend::components::header::HeaderProps;
use shared_frontend::components::footer::FooterProps;
use shared_frontend::i18n::Language;
use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    let site_title = "Publish".to_string();
    
    let header = HeaderProps {
        site_title,
        theme: "default".to_string(),
        language: Language::English,
        toggle_theme: Callback::from(|_| ()),
        on_language_change: Callback::from(|_| ()),
        is_authenticated: false,
        pin_required: false,
        on_logout: Callback::from(|_| ()),
        logout_tooltip: "".to_string(),
        theme_toggle_tooltip: "".to_string(),
        print_tooltip: "".to_string(),
        on_print: None,
        enable_translation: false,
        enable_themes: true,
        enable_print: false,
        print_disabled: true,
        site_url: None,
        repo: Some("publish".to_string()),
        version: None,
        version_url: None,
    };
    
    let footer = FooterProps {
        show_version: false,
        version: "".to_string(),
        show_github: true,
        github_url: None,
        version_url: None,
        repo: Some("publish".to_string()),
        show_coffee: false,
        coffee_url: None,
        children: html! {},
    };
    
    html! {
        <AppShell
            header={header}
            footer={footer}
            toasts={None}
            class={Classes::new()}
            main_class={Classes::new()}
            use_container={true}
        >
            <crate::editor::Editor initial_content="# Hello World\n\nStart typing here..." />
        </AppShell>
    }
}

pub fn run() {
    yew::Renderer::<App>::new().render();
}
