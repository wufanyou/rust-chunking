mod html;
mod similarity;

use pyo3::prelude::*;
use text_splitter::{ChunkConfig, TextSplitter};
use tokenizers::{Tokenizer, tokenizer};
use rayon::prelude::*;

#[pyclass(frozen, name = "MyTextSplitter")]
struct MyTextSplitter {
    splitter: TextSplitter<Tokenizer>,
    max_tokens: usize,
}

#[pymethods]
impl MyTextSplitter {
    #[new]
    #[pyo3(signature = (max_tokens, file))]
    fn new(max_tokens: usize, file: &str) -> PyResult<Self> {
        let tokenizer:Tokenizer = Tokenizer::from_file(file).unwrap();
        Ok(Self {
            splitter: TextSplitter::new(ChunkConfig::new(max_tokens).with_sizer(tokenizer)),
            max_tokens
        })
    }

    fn chunks<'text, 'splitter: 'text>(&'splitter self, text: &'text str) -> Vec<&'text str> {
        self.splitter.chunks(text).collect()
    }
    fn chunks_batch<'text, 'splitter: 'text>(&'splitter self, text: Vec<String>) -> Vec<Vec<String>>{
        let output:Vec<Vec<String>> = text.into_par_iter().map(|t:String| {
            let clean_string = html::convert(t.as_str(), self.max_tokens);
            let mut temp: Vec<String> = Vec::new();
            for c in self.splitter.chunks(clean_string.as_str()) {
                temp.push(String::from(c));
            }
            temp
        }).collect();
        similarity::test().expect("fail");
        output
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn rusk_chunking(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MyTextSplitter>()?;
    Ok(())
}
