// 패턴 인코딩: '#' = 살아있는 세포, '.' = 빈칸
pub struct Pattern {
    rows: &'static [&'static str],
}

impl Pattern {
    pub fn alive_cells(&self) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch == '#' {
                    cells.push((x, y));
                }
            }
        }
        cells
    }
}

pub const BLINKER: Pattern = Pattern { rows: &["###"] };

pub const TOAD: Pattern = Pattern {
    rows: &[".###", "###."],
};

pub const BEACON: Pattern = Pattern {
    rows: &["##..", "##..", "..##", "..##"],
};

pub const PULSAR: Pattern = Pattern {
    rows: &[
        "..###...###..",
        ".............",
        "#....#.#....#",
        "#....#.#....#",
        "#....#.#....#",
        "..###...###..",
        ".............",
        "..###...###..",
        "#....#.#....#",
        "#....#.#....#",
        "#....#.#....#",
        ".............",
        "..###...###..",
    ],
};

pub const GLIDER: Pattern = Pattern {
    rows: &[".#.", "..#", "###"],
};

pub const LWSS: Pattern = Pattern {
    rows: &[".#..#", "#....", "#...#", "####."],
};

pub const R_PENTOMINO: Pattern = Pattern {
    rows: &[".##", "##.", ".#."],
};
