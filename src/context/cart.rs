use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct CartItem {
    pub product_id: usize,
    pub title: String,
    pub unit_price: f32,
    pub quantity: u32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CartState {
    pub items: Vec<CartItem>,
}

impl CartState {
    pub fn add_item(&mut self, product_id: usize, title: String, unit_price: f32, quantity: u32) {
        if let Some(existing) = self.items.iter_mut().find(|item| item.product_id == product_id) {
            existing.quantity = existing.quantity.saturating_add(quantity);
            return;
        }

        self.items.push(CartItem {
            product_id,
            title,
            unit_price,
            quantity,
        });
    }

    pub fn remove_item(&mut self, product_id: usize) {
        self.items.retain(|item| item.product_id != product_id);
    }

    pub fn update_quantity(&mut self, product_id: usize, quantity: u32) {
        if quantity == 0 {
            self.remove_item(product_id);
            return;
        }
        if let Some(existing) = self.items.iter_mut().find(|item| item.product_id == product_id) {
            existing.quantity = quantity;
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn subtotal(&self) -> f32 {
        self.items
            .iter()
            .map(|item| item.unit_price * item.quantity as f32)
            .sum()
    }

    pub fn total_items(&self) -> u32 {
        self.items.iter().map(|item| item.quantity).sum()
    }
}

#[component]
pub fn CartProvider(children: Element) -> Element {
    use_context_provider(|| Signal::new(CartState::default()));
    rsx! { {children} }
}

#[cfg(test)]
mod tests {
    use super::CartState;

    #[test]
    fn add_item_appends_new_line_item() {
        let mut cart = CartState::default();
        cart.add_item(10, "Sneakers".to_string(), 99.0, 2);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 10);
        assert_eq!(cart.items[0].quantity, 2);
    }

    #[test]
    fn add_item_merges_quantity_for_existing_product() {
        let mut cart = CartState::default();
        cart.add_item(10, "Sneakers".to_string(), 99.0, 2);
        cart.add_item(10, "Sneakers".to_string(), 99.0, 3);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].quantity, 5);
    }

    #[test]
    fn remove_item_drops_only_selected_product() {
        let mut cart = CartState::default();
        cart.add_item(10, "Sneakers".to_string(), 99.0, 2);
        cart.add_item(20, "Bag".to_string(), 50.0, 1);

        cart.remove_item(10);

        assert_eq!(cart.items.len(), 1);
        assert_eq!(cart.items[0].product_id, 20);
    }

    #[test]
    fn update_quantity_replaces_existing_quantity() {
        let mut cart = CartState::default();
        cart.add_item(10, "Sneakers".to_string(), 99.0, 2);

        cart.update_quantity(10, 7);

        assert_eq!(cart.items[0].quantity, 7);
    }

    #[test]
    fn update_quantity_zero_removes_item() {
        let mut cart = CartState::default();
        cart.add_item(10, "Sneakers".to_string(), 99.0, 2);

        cart.update_quantity(10, 0);

        assert!(cart.items.is_empty());
    }

    #[test]
    fn subtotal_and_total_items_sum_all_lines() {
        let mut cart = CartState::default();
        cart.add_item(10, "Sneakers".to_string(), 99.0, 2);
        cart.add_item(20, "Bag".to_string(), 50.5, 3);

        assert!((cart.subtotal() - 349.5).abs() < f32::EPSILON);
        assert_eq!(cart.total_items(), 5);
    }

    #[test]
    fn clear_empties_cart() {
        let mut cart = CartState::default();
        cart.add_item(10, "Sneakers".to_string(), 99.0, 2);

        cart.clear();

        assert!(cart.items.is_empty());
        assert_eq!(cart.total_items(), 0);
        assert!((cart.subtotal() - 0.0).abs() < f32::EPSILON);
    }
}
