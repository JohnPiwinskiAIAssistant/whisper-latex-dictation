use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{
        request::Request,
        Content, Part, Role,
    },
};

pub struct GeminiClient {
    client: Client,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(api_key),
        }
    }

    pub async fn convert_to_latex(&self, prompt: &str) -> anyhow::Result<String> {
        let txt_request = Request {
            contents: vec![
                Content {
                    role: Role::User,
                    parts: vec![
                        Part {
                            text: Some(prompt.to_string()),
                            inline_data: None,
                            file_data: None,
                            video_metadata: None,
                        }
                    ],
                }
            ],
            tools: vec![],
            safety_settings: vec![],
            generation_config: None,
        };

        // Post the request to the Gemini API (timeout 30s)
        let post_result = self.client.post(30, &txt_request).await
            .map_err(|e| anyhow::anyhow!("Gemini API error: {:?}", e))?;

        let response = post_result.rest()
            .ok_or_else(|| anyhow::anyhow!("Expected REST response from Gemini, got something else"))?;

        // Extract the text from the response
        let latex = response.candidates.get(0)
            .ok_or_else(|| anyhow::anyhow!("No candidates in Gemini response"))?
            .content.parts.get(0)
            .ok_or_else(|| anyhow::anyhow!("No parts in Gemini response content"))?
            .text.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No text in Gemini response part"))?
            .clone();

        Ok(latex)
    }
}
