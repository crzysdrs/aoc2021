use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 9999;
    type Input = ();
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(_r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        unimplemented!()
    }
    fn p1(_v: &Self::Input) -> Self::Sol1 {
        unimplemented!()
    }
    fn p2(_v: &Self::Input) -> Self::Sol2 {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    //use super::*;
    #[test]
    fn test() {
        //unimplemented!()
    }
}
