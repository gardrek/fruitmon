pub struct Mon {
    level: u8,
    stats: [u8; 5],
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Stat {
    Strength = 0,
    Fortitude,
    Endurance,
    Agility,
    Will,

    Health,

    _Sentry,
}

impl From<u8> for Stat {
    fn from(index: u8) -> Self {
        if index >= (Stat::_Sentry as u8) {
            Stat::_Sentry
        } else {
            unsafe { std::mem::transmute::<u8, Stat>(index) }
        }
    }
}

impl From<Stat> for u8 {
    fn from(stat: Stat) -> Self {
        stat as u8
    }
}

impl Mon {
    pub fn new() -> Self {
        let level = 0;
        let stats = Default::default();

        Self { level, stats }
    }

    // Base stats

    pub fn base_stat(&self, stat: Stat) -> u8 {
        let index = stat as u8;
        if index < 5 {
            self.stats[index as usize]
        } else {
            1
        }
    }

    // Other stats derived from level, individual values, and base stats

    pub fn stat(&self, stat: Stat) -> u8 {
        let base = self.base_stat(stat);

        base
    }
}
