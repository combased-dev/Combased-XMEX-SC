The init method is used to set up the contract by defining certain variables such as the token identifier for the payment token, the nonce for the payment token.

The claim method allows users to claim a reward. Claim method verifies whether the address is in the WL, or already claimed the reward.

The fund method allows the contract owner to add funds to the contract by sending payment tokens to it.

The add_whitelist method allows owner to sets the eligble address to claim the reward.

The add_whitelist_with_xmex method allows owner to sets the eligble address &amount of reward per address to claim.

The set_xmex_per_address method is similar to add_whitelist_with_xmex, just that it sets individually.

The change_token_id changes the reward token.

The change_token_nonce changes the reward token nonce.
