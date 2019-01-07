use minifb::{Key, Scale, Window, WindowOptions};
use rand::{thread_rng, Rng};

const ROWS: usize = 90;
const COLS: usize = 160;

const COLORS: [u32; 36] = [
    0x0007_0707,
    0x001f_0707,
    0x002f_0f07,
    0x0047_0f07,
    0x0057_1707,
    0x0067_1f07,
    0x0077_1f07,
    0x008f_2707,
    0x009f_2f07,
    0x00af_3f07,
    0x00bf_4707,
    0x00c7_4707,
    0x00df_4f07,
    0x00df_5707,
    0x00df_5707,
    0x00d7_5f07,
    0x00d7_670f,
    0x00cf_6f0f,
    0x00cf_770f,
    0x00cf_7f0f,
    0x00cf_8717,
    0x00c7_8717,
    0x00c7_8f17,
    0x00c7_971f,
    0x00bf_9f1f,
    0x00bf_9f1f,
    0x00bf_a727,
    0x00bf_a727,
    0x00bf_af2f,
    0x00b7_af2f,
    0x00b7_b72f,
    0x00b7_b737,
    0x00cf_cf6f,
    0x00df_df9f,
    0x00ef_efc7,
    0x00ff_ffff,
];

#[derive(Clone, Copy)]
struct FirePixel {
    index: usize,
}

impl FirePixel {
    fn new() -> FirePixel {
        FirePixel { index: 0 }
    }
}

type FireGrid = [[FirePixel; COLS]; ROWS];

struct State {
    fire_grid: FireGrid,
}

impl State {
    /// Initialize a new state with a fire grid where all `FirePixel`s are black (index == 0), except for the first row,
    /// where all `FirePixels` are at full heat (index = MAX_COLOR).
    fn new() -> State {
        let mut fire_grid = [[FirePixel::new(); COLS]; ROWS];
        fire_grid[0] = [FirePixel {
            index: COLORS.len() - 1,
        }; COLS];

        State { fire_grid }
    }

    fn update(&mut self) {
        for y in 1..ROWS {
            for x in 0..COLS {
                spread_fire(y, x, &mut self.fire_grid)
            }
        }
    }

    fn draw(&self, buffer: &mut [u32]) {
        for (y, row) in self.fire_grid.iter().enumerate() {
            for (x, fire_pixel) in row.iter().enumerate() {
                let color = COLORS[fire_pixel.index];
                let y = ROWS - y - 1;
                buffer[x + y * COLS] = color;
            }
        }
    }
}

fn spread_fire(target_y: usize, target_x: usize, fire_grid: &mut FireGrid) {
    let mut rng = thread_rng();

    // heat source
    let src_index = {
        /* heat can go sideways, so we accept the following ranges:
        - y: [-1, 0]
        - x: [-1, +1] (must check boundaries)
        */
        let source_x = {
            let modifier = rng.gen_range(-1, 2);
            let cols = COLS as isize;
            ((target_x as isize + modifier + cols) % cols) as usize
            // or use mod_euc, which hasn't been stabilized yet
        };
        let source_y = target_y - rng.gen_range(0, 2);

        let source_fire_pixel = &fire_grid[source_y][source_x];
        source_fire_pixel.index
    };

    // fire pixel visited by this iteration
    let mut target_fire_pixel = &mut fire_grid[target_y][target_x];
    let decay = rng.gen_range(0, 2);
    target_fire_pixel.index = src_index.saturating_sub(decay);
}

pub fn main() -> minifb::Result<()> {
    let mut state = State::new();
    let mut buffer = [0; ROWS * COLS];
    let mut window = Window::new(
        "doom-fire",
        COLS,
        ROWS,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        state.update();
        state.draw(&mut buffer);
        window.update_with_buffer(&buffer)?;
    }

    Ok(())
}
