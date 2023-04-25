#[cfg(test)]
mod test {
    use crate::DepthUpdate;
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

    #[test]
    fn deserialize_depth_update() {
        let update = r#"
            {
                "e": "depthUpdate",
                "E": 123456789,
                "s": "BNBBTC",
                "U": 157,
                "u": 160,
                "b": [
                    [
                        "27826.89000000",
                        "2.50099000"
                    ],
                    [
                        "27826.10000000",
                        "0.69556000"
                    ]
                ],
                "a": [
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
        let book: DepthUpdate = serde_json::from_str(update).unwrap();
        let expected = DepthUpdate {
            timestamp: 123456789,
            first_update_id: 157,
            last_update_id: 160,
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
