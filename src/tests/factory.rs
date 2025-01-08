use alkanes::message::AlkaneMessageContext;
use alkanes_support::id::AlkaneId;
use anyhow::Result;
use bitcoin::blockdata::transaction::OutPoint;
use bitcoin::address::{NetworkChecked};
use bitcoin::{Witness, Sequence,  Amount, ScriptBuf, Script, Address, TxIn, TxOut, Transaction};
use protorune_support::protostone::Protostone;
use protorune::protostone::Protostones;
use protorune::message::MessageContextParcel;
use alkanes_support::trace::{Trace};
#[allow(unused_imports)]
use hex;
use metashrew_support::index_pointer::KeyValuePointer;
use alkanes_support::gz::{compress};
use alkanes::view;
#[allow(unused_imports)]
use metashrew_support::{utils::{format_key}};
use protorune::{test_helpers::{get_address}, balance_sheet::load_sheet, message::MessageContext, tables::RuneTable};

use protorune_support::utils::consensus_encode;

use alkanes::indexer::index_block;
use ordinals::{Runestone, Artifact};
use alkanes::tests::helpers as alkane_helpers;
use crate::tests::std::{free_mint_build};
use alkanes_support::{cellpack::Cellpack};
#[allow(unused_imports)]
use metashrew::{get_cache, index_pointer::IndexPointer, println, stdio::stdout};
use alkane_helpers::{clear};
use std::fmt::Write;
use alkanes::network::{set_view_mode};
use wasm_bindgen_test::wasm_bindgen_test;
use alkane_factory_support::constants::ALKANE_FACTORY_FREE_MINT_ID;


#[wasm_bindgen_test]
fn test_factory() -> Result<()> {
    clear();
    set_view_mode();
    let mut block_height = 850_000;
    let cellpacks: Vec<Cellpack> = [
        //auth token factory init
        Cellpack {
            target: AlkaneId { block: 3, tx: ALKANE_FACTORY_FREE_MINT_ID },
            inputs: vec![100]
        },
        Cellpack {
            target: AlkaneId { block: 6, tx: ALKANE_FACTORY_FREE_MINT_ID },
            inputs: vec![0, 100000, 100000, 100000000, 0x414243, 0x414243],
        }
    ]
    .into();
    let mut test_block = alkane_helpers::init_with_multiple_cellpacks_with_tx(
        [free_mint_build::get_bytes(), compress(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07])?].into(),
        cellpacks,
    );
    let len = test_block.txdata.len();
    let outpoint = OutPoint {
        txid: test_block.txdata[len - 1].compute_txid(),
        vout: 0
    };
    index_block(&test_block, block_height)?;
    let ptr = RuneTable::for_protocol(AlkaneMessageContext::protocol_tag())
        .OUTPOINT_TO_RUNES
        .select(&consensus_encode(&outpoint)?);
    let sheet = load_sheet(&ptr);
    println!("balances at end {:?}", sheet);
    println!("{:?}", <Vec<u8> as TryInto<Trace>>::try_into(view::trace(&OutPoint {
      txid: test_block.txdata[len - 1].compute_txid(),
      vout: 3
    })?)?);
    let mut parcel = MessageContextParcel::default();
    parcel.height = 850001;
    parcel.calldata = (Cellpack {
      target: AlkaneId {
        block: 2,
        tx: 1
      },
      inputs: vec![1000u128]
    }).serialize();
    println!("calldata: {:?}", &parcel.calldata);
    println!("{:?}", view::simulate_parcel(&parcel, u64::MAX)?.0);
    Ok(())
}
