#[derive(Clone, Debug)]
pub struct RelayXClient {
    api_key: String,
    base_url: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreparedRequest {
    pub method: &'static str,
    pub url: String,
    pub authorization: String,
    pub sdk_header: &'static str,
}

impl RelayXClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://gateway.relayx.vexarr.com".to_string(),
        }
    }

    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into().trim_end_matches('/').to_string();
        self
    }

    pub fn prepare_chat_completions(&self) -> PreparedRequest {
        self.prepare("POST", "/v1/chat/completions")
    }

    pub fn prepare_completions(&self) -> PreparedRequest {
        self.prepare("POST", "/v1/completions")
    }

    pub fn prepare_responses(&self) -> PreparedRequest {
        self.prepare("POST", "/v1/responses")
    }

    pub fn prepare_embeddings(&self) -> PreparedRequest {
        self.prepare("POST", "/v1/embeddings")
    }

    pub fn prepare_models(&self) -> PreparedRequest {
        self.prepare("GET", "/v1/models")
    }

    fn prepare(&self, method: &'static str, path: &str) -> PreparedRequest {
        PreparedRequest {
            method,
            url: format!("{}{}", self.base_url, path),
            authorization: format!("Bearer {}", self.api_key),
            sdk_header: "rust",
        }
    }
}

impl ChatMessage {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
}

impl ChatCompletionRequest {
    pub fn new(model: impl Into<String>, messages: Vec<ChatMessage>) -> Self {
        Self {
            model: model.into(),
            messages,
            stream: false,
        }
    }

    pub fn stream(mut self) -> Self {
        self.stream = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepares_openai_compatible_chat_endpoint() {
        let client = RelayXClient::new("rx_test").with_base_url("http://localhost:8080/");
        let request = client.prepare_chat_completions();
        assert_eq!(request.method, "POST");
        assert_eq!(request.url, "http://localhost:8080/v1/chat/completions");
        assert_eq!(request.authorization, "Bearer rx_test");
    }
}
