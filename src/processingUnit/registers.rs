#[derive(Copy, Clone)]
pub struct Registers {
    a: u8,
    f: u8,
    b:u8,
    c:u8,
    d:u8,
    e:u8,
    h:u8,
    l:u8,
    sp:u16,
    pc:u16,
}

impl Registers {
    pub fn access(self, address: &str) -> u8{
        match address {
            "a" => self.a,
            "f" => self.f,
            "b" => self.b,
            "c" => self.c, 
            "d" => self.d, 
            "e" => self.e, 
            "h" => self.h, 
            "l" => self.l,
            _ => panic!()
        }
    }
    pub fn update(mut self, address: &str, value: u8) {
        match address {
            "a" => self.a = value,
            "f" => self.f = value,
            "b" => self.b = value,
            "c" => self.c = value, 
            "d" => self.d = value, 
            "e" => self.e = value, 
            "h" => self.h = value, 
            "l" => self.l = value,
            _ => panic!()
    }
    }
}