use crate::layout_key::LayoutKey;

pub struct KeyMatrix {
    /// Abstracted key storage using LayoutKey
    pub keys: Vec<Vec<Vec<Option<LayoutKey>>>>,
    pub pressed: Vec<Vec<bool>>,
}

impl KeyMatrix {
    /// Create a KeyMatrix from pre-resolved LayoutKey data.
    pub fn from_layout_keys(
        keys: Vec<Vec<Vec<Option<LayoutKey>>>>,
        rows: usize,
        cols: usize,
    ) -> Self {
        KeyMatrix {
            keys,
            pressed: vec![vec![false; cols]; rows],
        }
    }

    pub fn get_num_layers(&self) -> usize {
        self.keys.len()
    }

    /// Get the LayoutKey at a position. Returns None for transparent keys.
    pub fn get_key(&self, layer: usize, row: usize, col: usize) -> Option<&LayoutKey> {
        self.keys
            .get(layer)
            .and_then(|l| l.get(row))
            .and_then(|r| r.get(col))
            .and_then(|k| k.as_ref())
    }

    /// Check if a key position is transparent (None = transparent).
    pub fn is_transparent(&self, layer: usize, row: usize, col: usize) -> bool {
        self.keys
            .get(layer)
            .and_then(|l| l.get(row))
            .and_then(|r| r.get(col))
            .map(|k| k.is_none())
            .unwrap_or(true)
    }

    pub fn is_pressed(&self, row: usize, col: usize) -> bool {
        self.pressed
            .get(row)
            .and_then(|r| r.get(col))
            .copied()
            .unwrap_or(false)
    }

    pub fn set_pressed(&mut self, row: usize, col: usize, value: bool) {
        if let Some(r) = self.pressed.get_mut(row) {
            if col < r.len() {
                r[col] = value;
            }
        }
    }
}
