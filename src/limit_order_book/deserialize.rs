#[cfg(test)]
mod test {
    use crate::LimitOrderBook;
    use crate::PriceAndQuantity;

    #[test]
    fn deserialize_from_snapshot() {
        let snapshot = r#"
            {
                "lastUpdateId": 17866404615,
                "bids": [
                    [
                        "27826.89000000",
                        "2.50099000"
                    ],
                    [
                        "27826.10000000",
                        "0.69556000"
                    ]
                ],
                "asks": [
                    [
                        "27826.90000000",
                        "4.80586000"
                    ],
                    [
                        "27826.91000000",
                        "0.26959000"
                    ]
                ]
            }
         "#;

        let book: LimitOrderBook = serde_json::from_str(snapshot).unwrap();
        let expected = LimitOrderBook {
            update_id: 17866404615,
            bids: vec![
                PriceAndQuantity(27826.1, 0.69556),
                PriceAndQuantity(27826.89, 2.50099),
            ]
            .into(),
            asks: vec![
                PriceAndQuantity(27826.91000000, 0.26959000),
                PriceAndQuantity(27826.90000000, 4.80586000),
            ]
            .into(),
        };
        assert_eq!(book, expected);
    }
}
