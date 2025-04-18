

#[cfg(test)]
mod tests {
    use crate::storage::layout::tembo_page::TemboPageZero;

    #[test]
    fn if_page_zero_size_aligns_with_the_rest_of_the_pages() {
        assert_eq!(size_of::<TemboPageZero>(), 8192);
    }
}
