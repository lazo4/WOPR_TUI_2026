pub mod anthropic;
pub mod budget;
pub mod cache;
pub mod minimax;
pub mod stub;
pub mod types;

use types::{LlmError, LlmRequest, LlmResponse, StreamResult};

pub trait LlmProvider: Send + Sync {
    fn generate(
        &self,
        request: &LlmRequest,
    ) -> impl std::future::Future<Output = Result<LlmResponse, LlmError>> + Send;

    fn generate_stream(&self, request: &LlmRequest) -> StreamResult;
}

use crate::config::Settings;

pub fn create_provider(settings: &Settings) -> Box<dyn LlmProviderBoxed> {
    // ponytail: route by protocol kind, not provider name — blumi providers all declare their wire protocol
    match settings.provider.as_str() {
        "anthropic" => {
            let key = settings
                .api_key
                .clone()
                .expect("provider requires api_key in ~/.blumi/settings.json");
            Box::new(anthropic::AnthropicProvider::new(
                key,
                settings.model.clone(),
                settings.base_url.clone(),
            ))
        }
        _ if settings.api_key.is_some() => {
            let key = settings.api_key.clone().unwrap();
            Box::new(anthropic::AnthropicProvider::new(
                key,
                settings.model.clone(),
                settings.base_url.clone(),
            ))
        }
        _ => Box::new(stub::StubProvider::new()),
    }
}

// ponytail: object-safe wrapper since LlmProvider uses RPITIT
pub trait LlmProviderBoxed: Send + Sync {
    fn generate_boxed<'a>(
        &'a self,
        request: &'a LlmRequest,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<LlmResponse, LlmError>> + Send + 'a>,
    >;

    fn generate_stream_boxed(&self, request: &LlmRequest) -> StreamResult;
}

impl LlmProviderBoxed for stub::StubProvider {
    fn generate_boxed<'a>(
        &'a self,
        request: &'a LlmRequest,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<LlmResponse, LlmError>> + Send + 'a>,
    > {
        Box::pin(self.generate(request))
    }

    fn generate_stream_boxed(&self, request: &LlmRequest) -> StreamResult {
        self.generate_stream(request)
    }
}

impl LlmProviderBoxed for anthropic::AnthropicProvider {
    fn generate_boxed<'a>(
        &'a self,
        request: &'a LlmRequest,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<LlmResponse, LlmError>> + Send + 'a>,
    > {
        Box::pin(self.generate(request))
    }

    fn generate_stream_boxed(&self, request: &LlmRequest) -> StreamResult {
        self.generate_stream(request)
    }
}

impl LlmProviderBoxed for minimax::MinimaxProvider {
    fn generate_boxed<'a>(
        &'a self,
        request: &'a LlmRequest,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<LlmResponse, LlmError>> + Send + 'a>,
    > {
        Box::pin(self.generate(request))
    }

    fn generate_stream_boxed(&self, request: &LlmRequest) -> StreamResult {
        self.generate_stream(request)
    }
}
