#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait Claim {
    #[init]
    fn init(
        &self,
        tokenid: EgldOrEsdtTokenIdentifier,
        nonce_id: u64,
    ) {
        self.token_id().set(&tokenid);
        self.nonce_token().set(nonce_id);
    }

    #[endpoint]
    #[payable("*")]
    fn claim(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            self.white_list_addresses().contains(&caller),
            "This address cannot claim, cause you are not in the WL for this batch"
        );
        require!(
            self.is_reward_claimed(&caller).get() == false,
            "This address cannot claim, because you already claimed"
        );
        let amount = self.reward_per_address(&caller).get();
        let nonce = self.nonce_token().get();
        self.is_reward_claimed(&caller).set(true);
        self.send()
            .direct(&caller, &self.token_id().get(), nonce, &amount);

    }

    #[only_owner]
    #[payable("*")]
    #[endpoint]
    fn fund(&self) {
        let payments = self.call_value().all_esdt_transfers();

        for payment in payments.into_iter() {
            self.nonce_token().set(payment.token_nonce); //set new nonce for lkmex
        }
    }

    #[only_owner]
    #[endpoint(addWhitelist)]
    fn add_whitelist(&self, addresses: ManagedVec<ManagedAddress>) -> SCResult<()> {
        for address in &addresses {
            self.white_list_addresses().insert(address.into());
        }

        Ok(())
    }

    /*
    Called every 2 weeks per owner to set the amount per address;
    */
    #[only_owner]
    #[endpoint(addWlWithXmex)]
    fn add_whitelist_with_xmex(
        &self,
        args: MultiValueEncoded<MultiValue2<ManagedAddress, BigUint>>
    ) {

        for arg in args.into_iter() {
            let (address, amount) = arg.into_tuple();
            self.white_list_addresses().insert(address.clone());
            self.reward_per_address(&address).set(&amount);
            self.is_reward_claimed(&address).set(false);
        }
    }

    #[only_owner]
    #[endpoint]
    fn change_token_id(&self, tokenid: EgldOrEsdtTokenIdentifier) {
        self.token_id().set(tokenid);
    }

    #[only_owner]
    #[endpoint]
    fn change_token_nonce(&self, token_nonce: u64) {
        self.nonce_token().set(token_nonce);
    }




    // STORAGE AREA
    #[view(getTokenId)]
    #[storage_mapper("tokenid")]
    fn token_id(&self) -> SingleValueMapper<EgldOrEsdtTokenIdentifier>;

    #[view(getNonceId)]
    #[storage_mapper("nonceid")]
    fn nonce_token(&self) -> SingleValueMapper<u64>;

    #[view(getIsRewardClaimed)]
    #[storage_mapper("rewardisclaimed")]
    fn is_reward_claimed(&self, address: &ManagedAddress) -> SingleValueMapper<bool>;

    #[view(getRewardPerAddress)]
    #[storage_mapper("rewardperaddress")]
    fn reward_per_address(&self, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getWhiteListedAddresses)]
    #[storage_mapper("whiteListAddresses")]
    fn white_list_addresses(&self) -> SetMapper<ManagedAddress>;

}
