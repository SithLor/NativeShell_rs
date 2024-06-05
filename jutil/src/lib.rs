
//create struct hold stream of byte data add read method to read data from stream
pub struct ByteStream<'a> {
    data: &'a [u8],
    index: usize,
}
impl ByteStream<'_> {
    pub fn new(data: &[u8]) -> ByteStream {
        ByteStream { data, index: 0 }
    }
    pub fn read(&mut self) -> Option<u8> {
        if self.index < self.data.len() {
            let byte = self.data[self.index];
            self.index += 1;
            Some(byte)
        } else {
            None
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
