mod html;
mod similarity;

use ammonia::clean_text;
use pyo3::prelude::*;
use text_splitter::{ChunkConfig, ChunkSizer, TextSplitter, Characters};
use tokenizers::{Tokenizer, tokenizer};
use rayon::prelude::*;

#[pyclass(frozen, name = "MyTextSplitter")]
struct MyTextSplitter {
    splitter: TextSplitter<Tokenizer>,
    // splitter:TextSplitter<Characters>,
    // max_tokens: usize,
}

#[pymethods]
impl MyTextSplitter {
    #[new]
    #[pyo3(signature = (max_tokens, overlap, file))]
    fn new(max_tokens: usize, overlap: usize, file: &str) -> PyResult<Self> {
        let tokenizer:Tokenizer = Tokenizer::from_file(file).unwrap();
        Ok(Self {
            splitter: TextSplitter::new(ChunkConfig::new(max_tokens).with_overlap(overlap).unwrap().with_sizer(tokenizer)),
        })
    }
    fn chunks<'text, 'splitter: 'text>(&'splitter self, text: &'text str) -> Vec<String> {
        let clean_string = html::convert(text);
        let mut temp: Vec<String> = Vec::new();
        for c in self.splitter.chunks(clean_string.as_str()) {
            temp.push(String::from(c));
        }
        temp
    }
    fn chunks_batch<'splitter>(&'splitter self, text: Vec<String>) -> Vec<Vec<String>>{
        let output:Vec<Vec<String>> = text.into_par_iter().map(|t:String| {
            self.chunks(t.as_str())
        }).collect();
        output
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn rusk_chunking(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MyTextSplitter>()?;
    Ok(())
}
