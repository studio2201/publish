use yew::prelude::*;
use pulldown_cmark::{Parser, html};
use web_sys::{HtmlTextAreaElement, EventTarget};
use wasm_bindgen::JsCast;
use crate::crdt::{CrdtBlock, sync_to_mock_network};

#[derive(Properties, PartialEq)]
pub struct EditorProps {
    pub initial_content: String,
}

pub struct Editor {
    pub content: String,
    pub compiled_html: String,
    pub block: CrdtBlock,
    pub sync_status: String,
}

pub enum Msg {
    UpdateContent(String),
    Sync,
}

impl Component for Editor {
    type Message = Msg;
    type Properties = EditorProps;

    fn create(ctx: &Context<Self>) -> Self {
        let initial = ctx.props().initial_content.clone();
        let block = CrdtBlock::new("doc-1", initial.clone());
        
        let compiled_html = compile_markdown(&initial);

        Self {
            content: initial,
            compiled_html,
            block,
            sync_status: "Unsynced".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateContent(new_content) => {
                self.content = new_content;
                self.compiled_html = compile_markdown(&self.content);
                self.block.update(&self.content);
                self.sync_status = "Unsynced".to_string();
                true
            }
            Msg::Sync => {
                match sync_to_mock_network(&self.block) {
                    Ok(_) => {
                        self.sync_status = "Synced".to_string();
                    }
                    Err(e) => {
                        self.sync_status = format!("Sync failed: {}", e);
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_input = ctx.link().callback(|e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = match target {
                Some(t) => t.dyn_into::<HtmlTextAreaElement>(),
                None => return Msg::UpdateContent(String::new()),
            };
            
            match input {
                Ok(textarea) => Msg::UpdateContent(textarea.value()),
                Err(_) => Msg::UpdateContent(String::new()),
            }
        });

        let on_sync = ctx.link().callback(|_| Msg::Sync);
        
        let html_content = Html::from_html_unchecked(AttrValue::from(self.compiled_html.clone()));

        html! {
            <div class="editor-container" style="display: flex; gap: 20px; height: 100%; min-height: 500px;">
                <div class="editor-pane" style="flex: 1; display: flex; flex-direction: column;">
                    <h2>{ "Editor" }</h2>
                    <textarea
                        value={self.content.clone()}
                        oninput={on_input}
                        style="flex: 1; resize: none; padding: 10px; font-family: monospace;"
                    />
                    <div style="margin-top: 10px;">
                        <button onclick={on_sync} style="padding: 5px 15px;">{ "Sync to P2P Node" }</button>
                        <span style="margin-left: 10px;">{ &self.sync_status }</span>
                    </div>
                </div>
                <div class="preview-pane" style="flex: 1; border: 1px solid #ccc; padding: 10px; overflow-y: auto;">
                    <h2>{ "Preview" }</h2>
                    <div>
                        { html_content }
                    </div>
                </div>
            </div>
        }
    }
}

fn compile_markdown(text: &str) -> String {
    let parser = Parser::new(text);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
