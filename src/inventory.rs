use bevy::prelude::*;

mod crafting;
use crafting::CraftingPlugin;

pub mod item;
use item::{Item, ItemPlugin};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ItemPlugin,
            CraftingPlugin
        ));
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ItemSlot {
    pub item: Option<Item>,
    pub amount: u32,
}

impl ItemSlot {
    fn new() -> Self {
        ItemSlot { item: None, amount: 0 }
    }

    pub fn clear(&mut self) {
        self.amount = 0;
        self.item = None;
    }
}

#[derive(Component, Debug)]
pub struct Inventory {
    pub items: Vec<ItemSlot>,
}

impl Inventory {
    pub fn new(_size: usize) -> Self {
        Self {
            items: vec![ItemSlot::new(); _size],
        }
    }

    pub fn has_room(&self, item: Item) -> bool {
        // проверяем, если слот с таким же типом предмета есть
        for slot in self.items.iter() {
            if slot.item == Some(item) {
                if slot.amount < item.max_stack {
                    return true;
                }
            }
        }

        // если нет, то пустой слот
        for slot in self.items.iter() {
            if slot.item.is_none() {
                return true;
            }
        }

        false
    }

    pub fn add_item(&mut self, item: Item) {
        // проверяем, если слот с таким же типом предмета есть
        for slot in self.items.iter_mut() {
            if slot.item == Some(item) {
                if slot.amount < item.max_stack {
                    slot.amount += 1;
                    return;
                }
            }
        }

        // если нет, то пустой слот
        for slot in self.items.iter_mut() {
            if slot.item.is_none() {
                slot.item = Some(item);
                slot.amount = 1;
                return;
            }
        }
    }

    pub fn has_item(&self, item: Item, amount: u32) -> bool {
        let mut sum = 0;

        for slot in self.items.iter() {
            if slot.item == Some(item) {
                sum += slot.amount;
            }
        }

        if sum >= amount {
            return true;
        }

        false
    }

    #[allow(unused)]
    pub fn remove_item(&mut self, item: Item) {
        for slot in self.items.iter_mut() {
            if slot.item == Some(item) {
                slot.amount -= 1;
                if slot.amount <= 0 {
                    slot.clear();
                }
                return;
            }
        }
    }

    pub fn remove_item_from_slot(&mut self, slot_id: usize) {
        let slot= &mut self.items[slot_id]; 
        slot.amount -= 1;
        if slot.amount <= 0 {
            slot.clear();
        }
    }
}