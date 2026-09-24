//! Listed EmbeddingModel trait + MockEmbedder (#946 / design #945).
//! Mock vectors are deterministic stand-ins. They are not semantic.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbeddingError {
    EmptyInput,
    ZeroDimension,
}

pub trait EmbeddingModel: Send + Sync {
    fn dimensions(&self) -> usize;
    fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError>;
    fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        texts.iter().copied().map(|t| self.embed(t)).collect()
    }
    fn model_id(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct MockEmbedder {
    dim: usize,
}

impl MockEmbedder {
    pub fn new(dim: usize) -> Result<Self, EmbeddingError> {
        if dim == 0 {
            return Err(EmbeddingError::ZeroDimension);
        }
        Ok(Self { dim })
    }
}

impl EmbeddingModel for MockEmbedder {
    fn dimensions(&self) -> usize {
        self.dim
    }

    fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        if text.trim().is_empty() {
            return Err(EmbeddingError::EmptyInput);
        }
        let mut vec = vec![0.0_f32; self.dim];
        for (i, b) in text.bytes().enumerate() {
            let idx = i % self.dim;
            vec[idx] += (b as f32) / 255.0;
        }
        let norm = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in &mut vec {
                *x /= norm;
            }
        }
        Ok(vec)
    }

    fn model_id(&self) -> &str {
        "mock-minilm-standin"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_rejects_zero_dim() {
        assert_eq!(MockEmbedder::new(0).err(), Some(EmbeddingError::ZeroDimension));
    }

    #[test]
    fn mock_rejects_empty() {
        let m = MockEmbedder::new(8).expect("dim");
        assert_eq!(m.embed("").err(), Some(EmbeddingError::EmptyInput));
        assert_eq!(m.embed("   ").err(), Some(EmbeddingError::EmptyInput));
    }

    #[test]
    fn mock_unit_norm_and_stable() {
        let m = MockEmbedder::new(8).expect("dim");
        let a = m.embed("amber tablet").expect("embed");
        let b = m.embed("amber tablet").expect("embed");
        assert_eq!(a, b);
        let norm: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 1e-5);
        assert_eq!(m.dimensions(), 8);
        assert_eq!(m.model_id(), "mock-minilm-standin");
    }

    #[test]
    fn batch_matches_sequential() {
        let m = MockEmbedder::new(4).expect("dim");
        let batch = m.embed_batch(&["one", "two"]).expect("batch");
        assert_eq!(batch[0], m.embed("one").expect("one"));
        assert_eq!(batch[1], m.embed("two").expect("two"));
    }
}
