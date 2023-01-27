# TBD

RUSTFLAGS='-C link-arg=-s' cargo build --all --target wasm32-unknown-unknown --release

cp target/wasm32-unknown-unknown/release/*.wasm ./res/

near dev-deploy --wasmFile ./res/filename.wasm

near deploy --accountId acc.testnet [--wasmFile ./path/filename.wasm]

lottery:
export CONTRACT=dev-1674056569009-24223185952985
export OWNER=bdrv.testnet
export ONE_NEAR=1000000000000000000000000
export THREE_NEAR=3000000000000000000000000
export FIVE_NEAR=5000000000000000000000000
export TEN_NEAR=10000000000000000000000000

export USER_1=sub.bdrv.testnet
export USER_2=sub2.bdrv.testnet
export USER_3=sub3.bdrv.testnet
export USER_4=sub4.bdrv.testnet
export USER_5=sub5.bdrv.testnet
export GAS=300000000000000

export ACCEPTED_SUBS=bdrv.testnet
export INVESTOR=bdrv2.testnet
export TREASURY=bdrv3.testnet

lottery_init:
near call $CONTRACT new '{
    "config": {
        "owner_id": "'$OWNER'",
        "contract_fee_ratio": 1000,
        "treasury_ratio": 0,
        "investor_ratio": 4000,
        "treasury": "'$TREASURY'",
        "investor": "'$INVESTOR'",
        "accepted_subs": "'$ACCEPTED_SUBS'"
    },
    "entry_fees": [
        ["near", [
            "'$ONE_NEAR'", 
            "'$THREE_NEAR'", 
            "'$FIVE_NEAR'"
        ]]
    ],
    "num_participants": [
        10
    ]
}' --accountId $CONTRACT

remove_num_participants  '{ 
    "num_participants": 5,
    "lottery_type": "SIMPLE_LOTTERY"
}' --accountId $CONTRACT


near view $CONTRACT get_lottery '{
    "lottery_id": 0
}'


pub fn draw_near_enter(
    &mut self, 
    //token_id: AccountId,
    lottery_type: String,
    num_participants: u32,
    entry_fee: U128
) -> LotteryId 