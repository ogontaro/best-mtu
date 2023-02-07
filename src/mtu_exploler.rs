use crate::mtu;
use crate::mtu::PingError;
use anyhow::Result;

pub struct MTUExplorer {
    pub mtu_range_max: u32,
    pub mtu_range_min: u32,
    pub next_mtu: u32,
    pub mtu_max: u32,
    pub mtu_min: u32,
}

impl Default for MTUExplorer {
    fn default() -> Self {
        Self {
            mtu_range_max: 1800,
            mtu_range_min: 1200,
            next_mtu: 0,
            mtu_max: 0,
            mtu_min: 0,
        }
    }
}

impl MTUExplorer {
    pub fn search_best_mtu(&mut self) -> Result<u32> {
        self.mtu_max = self.mtu_range_max - 28;
        self.mtu_min = self.mtu_range_min - 28;

        loop {
            if self.next().is_none() {
                break;
            };
            if self.is_more_big_mtu()? {
                self.mtu_min = self.next_mtu;
            } else {
                self.mtu_max = self.next_mtu;
            }
        }
        if self.mtu_max != self.mtu_min {
            if self.next_mtu == self.mtu_max {
                self.next_mtu = self.mtu_min;
            } else {
                self.next_mtu = self.mtu_max;
            }
            if self.is_more_big_mtu()? {
                self.next_mtu = self.mtu_max;
            } else {
                self.next_mtu = self.mtu_min;
            }
        }
        Ok(self.next_mtu + 28)
    }
    fn next(&mut self) -> Option<u32> {
        self.next_mtu = (self.mtu_max + self.mtu_min) / 2;
        if self.mtu_max - self.mtu_min <= 1 {
            return None;
        }
        Some(self.next_mtu)
    }
    fn is_more_big_mtu(&self) -> Result<bool> {
        print!("Checking MTU: {}", self.next_mtu + 28);
        let result = mtu::check(self.next_mtu);
        match result {
            Ok(_) => {
                println!(" OK");
                Ok(true)
            }
            Err(e) => match e.downcast_ref::<PingError>() {
                Some(e) => match e {
                    PingError::MessageTooLong(_) => {
                        println!(" NG: Size too long");
                        Ok(false)
                    }
                },
                None => Err(e),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check() {
        let mut explorer = MTUExplorer {
            mtu_range_min: 1499,
            mtu_range_max: 1503,
            ..Default::default()
        };
        let result = explorer.search_best_mtu();
        assert!(result.unwrap() == 1500);
    }
}
