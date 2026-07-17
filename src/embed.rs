//! OpenAI-compatible embeddings client (llama-server --embeddings).
//! Default local stack: Qwen3-Embedding-8B on 127.0.0.1:8302 (4096-d).

use nalgebra::DVector;
use serde_json::Value;
use std::time::Duration;

use crate::{unit_axis, LATENT_DIM_64};

pub const DEFAULT_EMBED_URL: &str = "http://127.0.0.1:8302/v1/embeddings";
/// Matches Qwen3-Embedding-8B GGUF n_embd on this machine.
pub const QWEN_EMBED_DIM: usize = 4096;

#[derive(Debug, Clone)]
pub struct EmbedClient {
    pub url: String,
    /// Model id as expected by the server (path or alias). Empty = server default.
    pub model: String,
    pub timeout_secs: u64,
}

impl Default for EmbedClient {
    fn default() -> Self {
        Self {
            url: DEFAULT_EMBED_URL.to_string(),
            model: String::new(),
            timeout_secs: 120,
        }
    }
}

impl EmbedClient {
    pub fn local_qwen() -> Self {
        Self {
            url: DEFAULT_EMBED_URL.to_string(),
            // llama-server accepts path-style model ids from /v1/models
            model: "/home/ruffianl/models/Qwen3-Embedding-8B-Q8_0.gguf".to_string(),
            timeout_secs: 120,
        }
    }

    /// Fetch one embedding vector (full server dim, usually 4096).
    pub fn embed(&self, text: &str) -> Result<DVector<f32>, String> {
        let mut body = serde_json::json!({ "input": text });
        if !self.model.is_empty() {
            body["model"] = Value::String(self.model.clone());
        }

        let agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(self.timeout_secs))
            .build();

        let resp = agent
            .post(&self.url)
            .set("Content-Type", "application/json")
            .send_json(body)
            .map_err(|e| format!("embed request failed: {e}"))?;

        let v: Value = resp
            .into_json()
            .map_err(|e| format!("embed json: {e}"))?;

        let arr = v
            .pointer("/data/0/embedding")
            .and_then(|x| x.as_array())
            .ok_or_else(|| format!("no data[0].embedding in response: {v}"))?;

        let mut out = Vec::with_capacity(arr.len());
        for x in arr {
            let f = x
                .as_f64()
                .ok_or_else(|| "non-float embedding component".to_string())? as f32;
            out.push(f);
        }
        Ok(DVector::from_vec(out))
    }
}

/// Project full embedding → 64D by averaging contiguous bins, then L2-normalize.
/// Deterministic, no extra matrix; matches "4096 → 64 forge" as a simple bridge.
pub fn project_to_64(v: &DVector<f32>) -> DVector<f32> {
    let n = v.len();
    if n == 0 {
        return DVector::zeros(LATENT_DIM_64);
    }
    if n == LATENT_DIM_64 {
        return unit_axis(v);
    }
    let mut out = vec![0.0f32; LATENT_DIM_64];
    for i in 0..LATENT_DIM_64 {
        let start = i * n / LATENT_DIM_64;
        let end = ((i + 1) * n / LATENT_DIM_64).max(start + 1);
        let slice = &v.as_slice()[start..end.min(n)];
        let mean = slice.iter().sum::<f32>() / slice.len() as f32;
        out[i] = mean;
    }
    unit_axis(&DVector::from_vec(out))
}

/// Embed text and project to unit 64D for PhysicsLang particles.
pub fn embed_to_64(client: &EmbedClient, text: &str) -> Result<DVector<f32>, String> {
    let full = client.embed(text)?;
    Ok(project_to_64(&full))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::XorShift32;

    #[test]
    fn project_4096_to_64_unit() {
        let mut rng = XorShift32::new(7);
        let v = rng.vec(4096);
        let p = project_to_64(&v);
        assert_eq!(p.len(), 64);
        assert!((p.norm() - 1.0).abs() < 1e-4);
    }
}
