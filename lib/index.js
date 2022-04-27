module.exports = ({ wallets, refs, config, client }) => ({
    getCount: () => client.query("counter", { get_count: {} }),
    increment: (signer = wallets.validator) =>
        client.execute(signer, "counter", { increment: {} }),
    mint: (signer = wallets.validator) =>
        client.execute(signer, "cw20_token", { mint: { "recipient": "terra15lgruwf5x6kazyjcgj4w9s0d0t5szx3l9epk5a", "amount": "1000000000" } }),
    balance: () => client.query("cw20_token", { balance: { "address": "terra15lgruwf5x6kazyjcgj4w9s0d0t5szx3l9epk5a" } }),
    getPrice: () => client.query("oracle", { query_price: {} }),
    updatePrice: (signer = wallets.validator) =>
        client.execute(signer, "oracle", { update_price: { "price": 102 } }),
});