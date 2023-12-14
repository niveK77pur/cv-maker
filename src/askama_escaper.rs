pub struct Tex;

impl askama_escape::Escaper for Tex {
    fn write_escaped<W>(&self, mut fmt: W, string: &str) -> core::fmt::Result
    where
        W: std::fmt::Write,
    {
        fmt.write_str(string)
    }
}
