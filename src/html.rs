use html2text::from_read;

pub(crate) fn convert(input:&str, width:usize) -> String{
    from_read(input.as_bytes(), width)
}