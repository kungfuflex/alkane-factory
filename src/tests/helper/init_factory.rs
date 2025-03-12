use crate::tests::std::free_mint_build;
use alkanes::tests::helpers::{self as alkane_helpers, assert_binary_deployed_to_id};
use alkanes_support::cellpack::Cellpack;
use alkanes_support::id::AlkaneId;
use anyhow::Result;
use bitcoin::Block;
use metashrew::stdio::stdout;
use std::fmt::Write;
use alkane_factory_support::constants::ALKANE_FACTORY_FREE_MINT_ID;

pub struct FreeMintDeploymentIds {
    pub free_mint_factory: AlkaneId,
}

pub fn init_free_mint_block() -> Result<(Block, FreeMintDeploymentIds)> {
    let cellpacks: Vec<Cellpack> = [
        // Free mint factory init
        Cellpack {
            target: AlkaneId {
                block: 3,
                tx: ALKANE_FACTORY_FREE_MINT_ID,
            },
            inputs: vec![100],
        },
    ]
    .into();
    
    let test_block = alkane_helpers::init_with_multiple_cellpacks_with_tx(
        [
            free_mint_build::get_bytes(),
        ]
        .into(),
        cellpacks,
    );
    
    // Define the deployment IDs
    let deployed_ids = FreeMintDeploymentIds {
        free_mint_factory: AlkaneId {
            block: 4,
            tx: ALKANE_FACTORY_FREE_MINT_ID,
        },
    };

    Ok((test_block, deployed_ids))
}

pub fn assert_free_mint_deployed(deployment_ids: &FreeMintDeploymentIds) -> Result<()> {
    let _ = assert_binary_deployed_to_id(
        deployment_ids.free_mint_factory.clone(),
        free_mint_build::get_bytes(),
    );
    
    let mut out = stdout();
    writeln!(out, "Free mint contract successfully deployed to {:?}", deployment_ids.free_mint_factory)?;
    Ok(())
}



