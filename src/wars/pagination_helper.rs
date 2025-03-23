#[cfg(test)]
mod sample_tests {
    struct PaginationHelper<T> {
        collection: Vec<T>,
        items_per_page: usize,
    }

    impl<T> PaginationHelper<T> {
        fn new(collection: Vec<T>, items_per_page: usize) -> Self {
            Self {
                collection,
                items_per_page,
            }
        }

        fn item_count(&self) -> usize {
            self.collection.len()
        }

        fn page_count(&self) -> usize {
            (self.item_count() as f64 / self.items_per_page as f64).ceil() as usize
        }

        fn page_item_count(&self, page_index: usize) -> Option<usize> {
            if page_index >= self.page_count() {
                return None;
            }

            let start = page_index * self.items_per_page;
            let end = (start + self.items_per_page).min(self.item_count());
            Some(end - start)
        }

        fn page_index(&self, item_index: usize) -> Option<usize> {
            if item_index >= self.item_count() {
                return None;
            }

            Some((item_index as f64 / self.items_per_page as f64).floor() as usize)
        }
    }

    #[test]
    fn test_item_count() {
        let helper = PaginationHelper::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 3);
        assert_eq!(helper.item_count(), 11);
    }

    #[test]
    fn test_page_count() {
        let helper = PaginationHelper::new(vec!['a', 'b', 'c', 'd', 'e', 'f'], 4);
        assert_eq!(helper.page_count(), 2);
    }

    #[test]
    fn test_page_item_count() {
        let helper = PaginationHelper::new((1..=24).collect(), 10);
        assert_eq!(helper.page_item_count(1), Some(10));
        assert_eq!(helper.page_item_count(2), Some(4));
    }

    #[test]
    fn test_page_index() {
        let helper = PaginationHelper::new((1..=24).collect(), 10);
        assert_eq!(helper.page_index(40), None);
        assert_eq!(helper.page_index(22), Some(2));
    }
}
