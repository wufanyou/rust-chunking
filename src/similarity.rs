// #[cfg(feature = "mkl")]
// extern crate intel_mkl_src;
//
// #[cfg(feature = "accelerate")]
// extern crate accelerate_src;
// use candle_transformers::models::bert::{BertModel, Config, HiddenAct, DTYPE};
// use anyhow::{Error as E, Result};
// use candle_core::{Tensor, Device, DType};
// use candle_nn::VarBuilder;
// use tokenizers::{PaddingParams, Tokenizer};
// use std::path::PathBuf;
//
// struct Args {
//     path: String,
//     revision: String,
//     use_pth: bool,
//     n: usize,
//     normalize_embeddings: bool,
//     approximate_gelu: bool,
// }
//
// impl Args {
//     fn build_model_and_tokenizer(&self) -> Result<(BertModel, Tokenizer)> {
//         let device = &Device::Cpu;
//         let model_id = &self.path;
//         // let repo = Cache::new(model_id);
//         let mut config = PathBuf::from(model_id.clone());
//         config.push("config.json");
//         let config = std::fs::read_to_string(config)?;
//         let mut config: Config = serde_json::from_str(&config)?;
//
//         let mut tokenizer = PathBuf::from(model_id.clone());
//         tokenizer.push("tokenizer.json");
//         let tokenizer = Tokenizer::from_file(tokenizer).map_err(E::msg)?;
//
//         let mut vb = PathBuf::from(model_id.clone());
//         vb.push("model.safetensors");
//         let vb = unsafe {VarBuilder::from_mmaped_safetensors(&[vb], DTYPE, &device)?};
//
//         if self.approximate_gelu {
//             config.hidden_act = HiddenAct::GeluApproximate;
//         }
//         let model = BertModel::load(vb, &config)?;
//         Ok((model, tokenizer))
//     }
// }
//
// pub(crate) fn test() -> Result<()> {
//
//     let args = Args{
//         path : "/Users/fanyou/Documents/KDD2024/bge-small-en-v1.5".to_string(),
//         revision : "abc".to_string(),
//         use_pth : false,
//         n : 1,
//         normalize_embeddings : true,
//         approximate_gelu : false,
//     };
//     let start = std::time::Instant::now();
//
//     let (model, mut tokenizer) = args.build_model_and_tokenizer()?;
//     let device = &model.device;
//     let sentences = [
//         "The cat sits outside",
//         "A man is playing guitar",
//         "I love pasta",
//         "The new movie is awesome",
//         "The cat plays in the garden",
//         "A woman watches TV",
//         "The new movie is so great",
//         "Do you like pizza?",
//     ];
//     let n_sentences = sentences.len();
//     if let Some(pp) = tokenizer.get_padding_mut() {
//         pp.strategy = tokenizers::PaddingStrategy::BatchLongest
//     } else {
//         let pp = PaddingParams {
//             strategy: tokenizers::PaddingStrategy::BatchLongest,
//             ..Default::default()
//         };
//         tokenizer.with_padding(Some(pp));
//     }
//     let tokens = tokenizer
//         .encode_batch(sentences.to_vec(), true)
//         .map_err(E::msg)?;
//     let token_ids = tokens
//         .iter()
//         .map(|tokens| {
//             let tokens = tokens.get_ids().to_vec();
//             Ok(Tensor::new(tokens.as_slice(), device)?)
//         })
//         .collect::<Result<Vec<_>>>()?;
//
//     let token_ids = Tensor::stack(&token_ids, 0)?;
//     let token_type_ids = token_ids.zeros_like()?;
//     println!("running inference on batch {:?}", token_ids.shape());
//     let embeddings = model.forward(&token_ids, &token_type_ids)?;
//
//
//     println!("generated embeddings {:?}", embeddings.shape());
//     // Apply some avg-pooling by taking the mean embedding value for all tokens (including padding)
//     let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
//     let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;
//     let embeddings = if args.normalize_embeddings {
//         normalize_l2(&embeddings)?
//     } else {
//         embeddings
//     };
//     println!("pooled embeddings {:?}", embeddings.shape());
//
//     let mut similarities = vec![];
//     for i in 0..n_sentences {
//         let e_i = embeddings.get(i)?;
//         for j in (i + 1)..n_sentences {
//             let e_j = embeddings.get(j)?;
//             let sum_ij = (&e_i * &e_j)?.sum_all()?.to_scalar::<f32>()?;
//             let sum_i2 = (&e_i * &e_i)?.sum_all()?.to_scalar::<f32>()?;
//             let sum_j2 = (&e_j * &e_j)?.sum_all()?.to_scalar::<f32>()?;
//             let cosine_similarity = sum_ij / (sum_i2 * sum_j2).sqrt();
//             similarities.push((cosine_similarity, i, j))
//         }
//     }
//     similarities.sort_by(|u, v| v.0.total_cmp(&u.0));
//     for &(score, i, j) in similarities[..5].iter() {
//         println!("score: {score:.2} '{}' '{}'", sentences[i], sentences[j])
//     }
//     Ok(())
// }
//
// pub fn normalize_l2(v: &Tensor) -> Result<Tensor> {
//     Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
// }