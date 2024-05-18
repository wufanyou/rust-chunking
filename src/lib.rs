use pyo3::prelude::*;
use text_splitter::{ChunkConfig, TextSplitter};
use tokenizers::Tokenizer;
use rayon::prelude::*;
fn split(text: &str) {
    let tokenizer = Tokenizer::from_file("./resource/tokenizer.json").unwrap();
    let max_tokens = 1;
    let splitter = TextSplitter::new(ChunkConfig::new(max_tokens).with_sizer(tokenizer));
    let chunks = splitter.chunks(text);
    for element in chunks {
        println!("the value is: {element}");
    }
}


#[pyclass(frozen, name = "MyTextSplitter")]
struct MyTextSplitter {
    splitter: TextSplitter<Tokenizer>,
    // tokenizer: Tokenizer
}


#[pymethods]
impl MyTextSplitter {
    #[new]
    #[pyo3(signature = (max_tokens, file))]
    fn new(max_tokens: usize, file: &str) -> PyResult<Self> {
        Ok(Self {
            splitter: TextSplitter::new(ChunkConfig::new(max_tokens).with_sizer(Tokenizer::from_file(file).unwrap())),
        })
    }

    fn chunks<'text, 'splitter: 'text>(&'splitter self, text: &'text str) -> Vec<&'text str> {
        self.splitter.chunks(text).collect()
    }
    fn chunks_batch<'text, 'splitter: 'text>(&'splitter self, text: Vec<String>) -> Vec<Vec<String>>{
        let output:Vec<Vec<String>> = text.into_par_iter().map(|t:String| {
            let mut temp: Vec<String> = Vec::new();
            for c in self.splitter.chunks(t.as_str()) {
                temp.push(String::from(c));
            }
            temp
        }).collect();
        return output
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn rusk_chunking(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MyTextSplitter>()?;
    Ok(())
}
