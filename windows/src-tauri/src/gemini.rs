// Gemini AI client
use std::sync::Mutex;
use serde_json::{json, Value};
use crate::secrets;
use crate::claude::{ChatContext, ChatReply};

const ENDPOINT_BASE: &str = "https://generativelanguage.googleapis.com/v1beta/models";
pub const DEFAULT_MODEL: &str = "gemini-2.0-flash";

pub const SYSTEM_PROMPT: &str = "Você é Mochi, um assistente de IA pessoal que vive no topo da tela do usuário. \
Você tem acesso à web e pode ajudar com absolutamente qualquer coisa — pesquisa, código, recomendações, tarefas. \
Quando solicitado, atue como um Social Media Manager (Gerenciador de Redes Sociais), analisando o público e sugerindo ideias de posts criativos, respostas ou estratégias de engajamento. \
Responda sempre em Português Brasileiro. Seja completo e detalhado. \
Sem formatação markdown. Use texto simples com quebras de linha.";

#[derive(Default)]
pub struct GeminiChat {
    pub messages: Mutex<Vec<Value>>,
}

impl GeminiChat {
    pub fn reset(&self) {
        self.messages.lock().unwrap().clear();
    }
    fn snapshot(&self) -> Vec<Value> {
        self.messages.lock().unwrap().clone()
    }
    fn push(&self, message: Value) {
        self.messages.lock().unwrap().push(message);
    }
}

pub async fn send(
    chat: &GeminiChat,
    model: &str,
    query: String,
    context: Option<ChatContext>,
) -> Result<ChatReply, String> {
    let key = secrets::get("gemini-api-key")
        .ok_or_else(|| "Chave Gemini não configurada. Abra as configurações.".to_string())?;

    let mut contents = chat.snapshot();
    let mut parts = vec![json!({"text": query})];
    
    if let Some(ctx) = context {
        match ctx {
            ChatContext::File { name, path } => {
                parts.insert(0, json!({"text": format!("Arquivo: {}\nCaminho: {}\n", name, path)}));
            }
            ChatContext::Window { app_name, title, .. } => {
                parts.insert(0, json!({"text": format!("Janela: {} - {}\n", app_name, title)}));
            }
        }
    }
    
    let user_msg = json!({
        "role": "user",
        "parts": parts
    });
    contents.push(user_msg.clone());

    let payload = json!({
        "systemInstruction": {
            "parts": [ { "text": SYSTEM_PROMPT } ]
        },
        "contents": contents,
    });

    let client = reqwest::Client::new();
    let url = format!("{}/{}:generateContent?key={}", ENDPOINT_BASE, model, key);
    
    let res = client.post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = res.status();
    let body: Value = res.json().await.map_err(|e| e.to_string())?;

    if !status.is_success() {
        if let Some(err_msg) = body["error"]["message"].as_str() {
            return Err(err_msg.to_string());
        }
        return Err(format!("Erro da API Gemini: {}", status));
    }

    let text = body["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string();

    chat.push(user_msg);
    chat.push(json!({
        "role": "model",
        "parts": [{ "text": text.clone() }]
    }));

    Ok(ChatReply { text })
}
