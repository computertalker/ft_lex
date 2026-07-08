pub struct Opt
{
    v: bool,
    n: bool,
    t: bool,
    stdin: bool,
}

impl Opt
{
    pub fn setv(&mut self) {
        self.v = true;
    }
    pub fn setn(&mut self) {
        self.n = true;
    }
    pub fn sett(&mut self) {
        self.t = true;
    }
    pub fn setstdin(&mut self) {
        self.stdin = true;
    }
    pub fn is_none(&mut self) -> bool {
        if self.v == false &&
            self.n == false &&
            self.t == false &&
            self.stdin == false {
            return true;
        }
        false
    }
}

impl Default for Opt
{
    fn default() -> Self {
        Self { v: false, n: false, t: false, stdin: false }
    }
}

