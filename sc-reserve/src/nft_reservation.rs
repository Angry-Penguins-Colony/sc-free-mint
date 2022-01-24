#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod whitelist;


#[elrond_wasm::contract]
pub trait NFTReservation:
  whitelist::WhitelistModule 
{
  #[init]
  fn init(&self,
      nft_price: BigUint,
      nft_total_amount: BigUint,
      max_reservation_per_transactions: BigUint,
      max_reservation_per_wallet:BigUint,
      unlock_timestamp: u64,
      delta_whitelist_timestamp: u64) -> SCResult<()>
  {
    require!(nft_price > 0, "NFT price cannot be set to zero");
    self.nft_price().set(&nft_price);

    require!(nft_total_amount > 0, "NFT total amount cannot be set to zero");
    self.nft_total_amount().set(&nft_total_amount);
    self.nft_remaining_amount().set(&nft_total_amount);

    require!(max_reservation_per_transactions > 0, "Max reservation per transactions cannot be set to zero");
    self.max_reservation_per_transactions().set(&max_reservation_per_transactions);

    self.max_reservation_per_wallet().set(&max_reservation_per_wallet);
    self.unlock_timestamp().set(&unlock_timestamp);

    let whitelist_unlock_timestamp = unlock_timestamp - delta_whitelist_timestamp;
    self.whitelist_unlock_timestamp().set(&whitelist_unlock_timestamp);    

    Ok(())
  }

  #[payable("EGLD")]
  #[endpoint(reserveNft)]
  fn reserve_nft(
    &self,
    #[payment_token] payment_token: TokenIdentifier,
    #[payment_amount] payment_amount: BigUint,
  ) -> SCResult<()> {


    let caller = self.blockchain().get_caller();

    if (self.is_whitelisted(&caller) == true) {
      require!(self.blockchain().get_block_timestamp() >= self.whitelist_unlock_timestamp().get(), "Whitelist unlock timestamp has not been reached");    
    }
    else {
      require!(self.blockchain().get_block_timestamp() >= self.unlock_timestamp().get(), "The reservation is not possible yet. Check on the website to get more informations.");
    }

    // check PAYMENT VALIDITY
    require!(&payment_token == &TokenIdentifier::egld(), "Invalid payment token");
    
    let price = self.nft_price().get();

    require!(payment_amount > 0, "Invalid payment amount. Must be greater than zero.");
    require!(&payment_amount % &price == 0, "Invalid payment amount. Must be a multiple of the nft price");

    // check we there is NFT REMAINING 
    let nft_to_buy = &payment_amount / &price;
    let remaining_amount = self.nft_remaining_amount().get();

    require!(&remaining_amount >= &nft_to_buy, "Sorry. There is not NFT remaining."); // is there remaining

    // check if transac doesn't exceed max per TRANSACTION
    require!(&nft_to_buy <= &self.max_reservation_per_transactions().get(), "Too much nft buyed from the same transactions. Try reduce it.");

    // check if transac doesn't exceed max per WALLET
    let current_nft_reserved = self.reserved_nft_amount(&caller).get();

    let max_reservation_per_wallet = self.max_reservation_per_wallet().get();
    let reserved_nft_amount = &current_nft_reserved + &nft_to_buy;
    
    require!(max_reservation_per_wallet == 0 || reserved_nft_amount <= max_reservation_per_wallet, "You exceed the maximum NFT you can own per wallet in this transaction. Try reduce it.");
    
    // reserve NFT
    self.reserved_nft_amount(&caller).set(&reserved_nft_amount); // increment reserved nft

    let nft_remaining_amount = &remaining_amount - &nft_to_buy;
    self.nft_remaining_amount().set(&nft_remaining_amount); // decrement remaining nft

    Ok(())
  }

  #[view(isWhitelisted)]
  fn is_whitelisted(&self, address: &ManagedAddress) -> bool {
    self.whitelist().contains(address)
  }

  #[endpoint]
  #[only_owner]
  fn claim(&self) -> SCResult<()> {

    // only_owner!(self, "Only owner may call this function");
    
    // STEP 1 : get balance
    let balance = self.blockchain().get_sc_balance(&TokenIdentifier::egld(), 0);
    
    // STEP 2 : require balance > 0
    require!(balance > 0, "There is nothing to claim. The balance is empty.");

    // STEP 3 : send balance to owner    
    let owner = self.blockchain().get_owner_address();
    self.send().direct_egld(&owner, &balance, &[]);

    Ok(())
  }

  #[view(getNftPrice)]
  #[storage_mapper("nftPrice")]
  fn nft_price(&self) -> SingleValueMapper<BigUint>;

  #[view(getNftTotalAmount)]
  #[storage_mapper("nftTotalAmount")]
  fn nft_total_amount(&self) -> SingleValueMapper<BigUint>;

  #[view(getNftRemainingAmount)]
  #[storage_mapper("nftRemainingAmount")]
  fn nft_remaining_amount(&self) -> SingleValueMapper<BigUint>;

  #[view(getMaxReservationPerTransactions)]
  #[storage_mapper("maxReservationPerTransactions")]
  fn max_reservation_per_transactions(&self) -> SingleValueMapper<BigUint>;

  #[view(getMaxReservationPerWallet)]
  #[storage_mapper("maxReservationPerWallet")]
  fn max_reservation_per_wallet(&self) -> SingleValueMapper<BigUint>;

  #[view(getReservedNFT)]
  #[storage_mapper("reservedNFT")]
  fn reserved_nft_amount(&self, adress: &ManagedAddress) -> SingleValueMapper<BigUint>;

  #[view(getUnlockTimestamp)]
  #[storage_mapper("unlockTimestamp")]
  fn unlock_timestamp(&self) -> SingleValueMapper<u64>;

  #[view(getWhitelistUnlockTimestamp)]
  #[storage_mapper("whitelist_unlock_timestamp")]
  fn whitelist_unlock_timestamp(&self) -> SingleValueMapper<u64>;
}
