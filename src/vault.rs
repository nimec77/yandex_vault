use std::{collections::HashMap, fmt::format};

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub size: u32,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub items: Vec<Item>,
    pub capacity: u32,
    pub used_space: u32,
}

#[derive(Debug)]
pub enum CellError {
    Full,
}

impl Cell {
    pub fn new(capacity: u32) -> Self {
        Self {
            items: Vec::new(),
            capacity,
            used_space: 0,
        }
    }

    pub fn put_item(&mut self, item: Item) -> Result<(), CellError> {
        if self.used_space + item.size > self.capacity {
            return Err(CellError::Full);
        }
        self.used_space += item.size;
        self.items.push(item);
        Ok(())
    }

    pub fn list_items(&self) -> Option<String> {
        if self.items.is_empty() {
            None
        } else {
            let description: Vec<String> = self
                .items
                .iter()
                .map(|item| format!("{}: {}", item.name, item.size))
                .collect();

            Some(format!(
                "Items: {} | Used: {}/{}\n",
                description.join(", "),
                self.used_space,
                self.capacity,
            ))
        }
    }
}

pub struct Vault {
    pub cells: HashMap<u32, Cell>,
    pub capacity: usize,
}

#[derive(Debug)]
pub enum VaultError {
    VaultFull,
    CellFull,
    CellNotFound,
}

impl Vault {
    pub fn new(capacity: usize) -> Self {
        Self {
            cells: HashMap::new(),
            capacity,
        }
    }

    pub fn put(&mut self, id: u32, item: Item, cell_capacity: u32) -> Result<(), VaultError> {
        if self.cells.len() >= self.capacity && !self.cells.contains_key(&id) {
            return Err(VaultError::VaultFull);
        }

        let cell = self
            .cells
            .entry(id)
            .or_insert_with(|| Cell::new(cell_capacity));
        cell.put_item(item).map_err(|_| VaultError::CellFull)
    }
}
