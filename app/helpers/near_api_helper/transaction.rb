class Transaction 
  include Borsh

  borsh signer_id: :string, public_key: :string, 
  nonce: :u64, receiver_id: :string, block_hash: Array, 
  actions: Array
end