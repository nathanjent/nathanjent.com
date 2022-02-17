#[derive(QueryParams)]
pub struct PageRequest {
    pub page: i32,
}

#[cfg(test)]
mod test {
    use super::PageRequest;

    #[test]
    fn test() {
        let pr = PageRequest {
            page: 2,
        };

        assert_eq!("?page=2", pr.to_query_params());
    }
}
