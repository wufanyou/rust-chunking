use pyo3::prelude::*;
use pyo3::types::PyList;
use text_splitter::{ChunkConfig, TextSplitter};
use tokenizers::Tokenizer;


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
    // TODO: make praliline happened
    fn chunks_batch<'text, 'splitter: 'text>(&'splitter self, text: Vec<Vec<u8>>) {
        for x in text {
            let string = String::from_utf8(x).expect("Our bytes should be valid utf8");
            println!("{string}")
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rusk_chunking(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MyTextSplitter>()?;
    Ok(())
}
