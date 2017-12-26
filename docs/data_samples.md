
# Bitstamp JSON examples

api.ticker(Pair::BTC_USD)

```rust    
    { timestamp: 1513866015986,
    pair: BTC_USD,
    last_trade_price: BigDecimal { int_val: BigInt { sign: Plus, data: BigUint { data: [1634990] } }, scale: 2 },
    lowest_ask: BigDecimal { int_val: BigInt { sign: Plus, data: BigUint { data: [1634995] } }, scale: 2 },
    highest_bid: BigDecimal { int_val: BigInt { sign: Plus, data: BigUint { data: [1634985] } }, scale: 2 },
    volume: Some(BigDecimal { int_val: BigInt { sign: Plus, data: BigUint { data: [740003355, 509] } }, scale: 8 }) }
```
