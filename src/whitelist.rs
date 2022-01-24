elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait WhitelistModule {
    #[only_owner]
    #[endpoint(whitelistAddress)]
    fn whitelist_address(&self, address: ManagedAddress) -> SCResult<()> {
        let is_new = self.whitelist().insert(address);
        require!(is_new, "Address already whitelisted");
        Ok(())
    }

    #[only_owner]
    #[endpoint(removeWhitelistedAddress)]
    fn remove_whitelist(&self, address: ManagedAddress) -> SCResult<()> {
        let is_removed = self.whitelist().remove(&address);
        require!(is_removed, "Addresss not whitelisted");
        Ok(())
    }

    fn require_whitelisted(&self, caller: &ManagedAddress) -> SCResult<()> {
        require!(self.whitelist().contains(caller), "Not whitelisted");
        Ok(())
    }

    #[view(getWhitelistedAddresses)]
    #[storage_mapper("whitelist")]
    fn whitelist(&self) -> SetMapper<ManagedAddress>;

    #[view(getWhitelistedManagedAddresses)]
    fn get_whitelisted_managed_addresses(&self) -> ManagedMultiResultVec<ManagedAddress> {
        let mut result = ManagedMultiResultVec::new(self.type_manager());
        for pair in self.whitelist().iter() {
            result.push(pair);
        }
        result
    }
}
