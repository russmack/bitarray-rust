

pub struct BitArray {
    words: u64,
}

impl BitArray {
    pub fn new() -> BitArray {
        return BitArray { words: 0 };
    }

    pub fn as_number(&self) -> u64 {
        return self.words;
    }

    pub fn from_number(&mut self, n: u64) {
        self.words = n;
    }

    pub fn from_binary(&mut self, s: &str) {
        let mut b: u64 = 0;

        let mut i = 0;
        for c in s.as_bytes().iter() {
            if i > 0 {
                b = b << 1;
            }

            if c == &49 {
                b |= 1
            }

            i += 1;
        }

        self.words = b;
    }

    pub fn as_string(&self) -> String {
        format!("{:b}", self.words)
    }

    pub fn set(&mut self, n: u64, b: bool) -> &mut BitArray {
        if b {
            self.words |= 1 << n;
        } else {
            self.words &= !(1 << n);
        }
        self
    }

    pub fn get(&self, n: u64) -> bool {
        let b = self.words & (1 << n);

        match b {
            0 => false,
            _ => true,
        }
    }

    pub fn flip(&mut self, n: u64) {
        self.words ^= 1 << n;
    }
}



#[cfg(test)]
mod tests {

    use super::BitArray;

    #[test]
    fn as_number() {
        struct TestCase {
            input: u64,
            expect: u64,
        }

        let testcases: [TestCase; 7] = [TestCase {
                                            input: 0,
                                            expect: 0,
                                        },
                                        TestCase {
                                            input: 1,
                                            expect: 1,
                                        },
                                        TestCase {
                                            input: 2,
                                            expect: 2,
                                        },
                                        TestCase {
                                            input: 10,
                                            expect: 10,
                                        },
                                        TestCase {
                                            input: 123,
                                            expect: 123,
                                        },
                                        TestCase {
                                            input: 255,
                                            expect: 255,
                                        },
                                        TestCase {
                                            input: 1024,
                                            expect: 1024,
                                        }];

        for t in testcases.iter() {
            let mut a = BitArray::new();
            a.from_number(t.input);
            assert_eq!(a.as_number(), t.expect);
        }
    }


    #[test]
    fn from_number() {
        struct TestCase {
            input: u64,
            expect: u64,
        }

        let testcases: [TestCase; 4] = [TestCase {
                                            input: 0,
                                            expect: 0,
                                        },
                                        TestCase {
                                            input: 1,
                                            expect: 1,
                                        },
                                        TestCase {
                                            input: 37,
                                            expect: 37,
                                        },
                                        TestCase {
                                            input: 255,
                                            expect: 255,
                                        }];

        for t in testcases.iter() {
            let mut a = BitArray::new();
            a.from_number(t.input);
            assert_eq!(a.as_number(), t.expect);
        }
    }

    #[test]
    fn from_binary() {
        struct TestCase {
            input: String,
            expect: u64,
        }

        let testcases: [TestCase; 11] = [TestCase {
                                             input: "".to_string(),
                                             expect: 0,
                                         },
                                         TestCase {
                                             input: "x".to_string(),
                                             expect: 0,
                                         },
                                         TestCase {
                                             input: "0".to_string(),
                                             expect: 0,
                                         },
                                         TestCase {
                                             input: "1".to_string(),
                                             expect: 1,
                                         },
                                         TestCase {
                                             input: "10".to_string(),
                                             expect: 2,
                                         },
                                         TestCase {
                                             input: "11".to_string(),
                                             expect: 3,
                                         },
                                         TestCase {
                                             input: "101".to_string(),
                                             expect: 5,
                                         },
                                         TestCase {
                                             input: "00001000".to_string(),
                                             expect: 8,
                                         },
                                         TestCase {
                                             input: "00000000".to_string(),
                                             expect: 0,
                                         },
                                         TestCase {
                                             input: "11111111".to_string(),
                                             expect: 255,
                                         },
                                         TestCase {
                                             input: "10011011".to_string(),
                                             expect: 155,
                                         }];

        for t in testcases.iter() {
            let mut a = BitArray::new();
            a.from_binary(&t.input);
            assert_eq!(a.as_number(), t.expect);
        }
    }

    #[test]
    fn as_string() {
        struct TestCase {
            input: u64,
            expect: String,
        }

        let testcases: [TestCase; 9] = [TestCase {
                                            input: 0,
                                            expect: "0".to_string(),
                                        },
                                        TestCase {
                                            input: 1,
                                            expect: "1".to_string(),
                                        },
                                        TestCase {
                                            input: 2,
                                            expect: "10".to_string(),
                                        },
                                        TestCase {
                                            input: 3,
                                            expect: "11".to_string(),
                                        },
                                        TestCase {
                                            input: 5,
                                            expect: "101".to_string(),
                                        },
                                        TestCase {
                                            input: 8,
                                            expect: "1000".to_string(),
                                        },
                                        TestCase {
                                            input: 0,
                                            expect: "0".to_string(),
                                        },
                                        TestCase {
                                            input: 255,
                                            expect: "11111111".to_string(),
                                        },
                                        TestCase {
                                            input: 155,
                                            expect: "10011011".to_string(),
                                        }];

        for t in testcases.iter() {
            let mut a = BitArray::new();
            a.from_number(t.input);
            assert_eq!(a.as_string(), t.expect);
        }
    }

    #[test]
    fn set_on() {
        struct TestCase {
            input: u64,
            expect: u64,
        }

        let testcases: [TestCase; 4] = [TestCase {
                                            input: 0,
                                            expect: 1,
                                        },
                                        TestCase {
                                            input: 1,
                                            expect: 2,
                                        },
                                        TestCase {
                                            input: 2,
                                            expect: 4,
                                        },
                                        TestCase {
                                            input: 3,
                                            expect: 8,
                                        }];

        for t in testcases.iter() {
            let mut a = BitArray::new();
            a.set(t.input, true);
            assert_eq!(a.as_number(), t.expect);
        }
    }

    #[test]
    fn set_off() {
        struct TestCase {
            input: u64,
            expect: u64,
        }

        let testcases: [TestCase; 4] = [TestCase {
                                            input: 0,
                                            expect: 170,
                                        },
                                        TestCase {
                                            input: 1,
                                            expect: 168,
                                        },
                                        TestCase {
                                            input: 2,
                                            expect: 170,
                                        },
                                        TestCase {
                                            input: 3,
                                            expect: 162,
                                        }];

        for t in testcases.iter() {
            let mut a = BitArray::new();
            a.from_number(170);
            a.set(t.input, false);
            assert_eq!(a.as_number(), t.expect);
        }
    }

    #[test]
    fn get() {
        struct TestCase {
            input: u64,
            expect: bool,
        }

        let testcases: [TestCase; 7] = [TestCase {
                                            input: 0,
                                            expect: true,
                                        },
                                        TestCase {
                                            input: 1,
                                            expect: false,
                                        },
                                        TestCase {
                                            input: 2,
                                            expect: true,
                                        },
                                        TestCase {
                                            input: 3,
                                            expect: false,
                                        },
                                        TestCase {
                                            input: 4,
                                            expect: true,
                                        },
                                        TestCase {
                                            input: 5,
                                            expect: true,
                                        },
                                        TestCase {
                                            input: 6,
                                            expect: true,
                                        }];

        let mut a = BitArray::new();
        a.set(0, true)
            .set(2, true)
            .set(4, true)
            .set(5, true)
            .set(6, true);

        for t in testcases.iter() {
            let b = a.get(t.input);
            assert_eq!(b, t.expect);
        }
    }

    #[test]
    fn flip() {
        struct TestCase {
            input: (u64, u64),
            expect: u64,
        }

        let testcases: [TestCase; 6] = [TestCase {
                                            input: (0, 0),
                                            expect: 1,
                                        },
                                        TestCase {
                                            input: (1, 0),
                                            expect: 0,
                                        },
                                        TestCase {
                                            input: (2, 0),
                                            expect: 3,
                                        },
                                        TestCase {
                                            input: (5, 1),
                                            expect: 7,
                                        },
                                        TestCase {
                                            input: (13, 2),
                                            expect: 9,
                                        },
                                        TestCase {
                                            input: (255, 7),
                                            expect: 127,
                                        }];

        for t in testcases.iter() {
            let mut a = BitArray::new();
            a.from_number(t.input.0);
            a.flip(t.input.1);
            assert_eq!(a.as_number(), t.expect);
        }
    }
}
