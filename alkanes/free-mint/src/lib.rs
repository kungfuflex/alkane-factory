use alkane_factory_support::factory::{MintableToken};
use anyhow::{anyhow, Result};
use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_support::{response::CallResponse, utils::shift_or_err};
use alkanes_support::utils::{overflow_error};
use alkanes_runtime::storage::{StoragePointer};
use metashrew_support::index_pointer::{KeyValuePointer};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};

#[derive(Default)]
pub struct MintableAlkane(());


impl MintableToken for MintableAlkane {}

impl MintableAlkane {
  pub fn minted_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/minted")
  }
  pub fn minted(&self) -> u128 {
    self.minted_pointer().get_value::<u128>()
  }
  pub fn set_minted(&self, v: u128) {
    self.minted_pointer().set_value::<u128>(v);
  }
  pub fn increment_mint(&self) -> Result<()> {
    self.set_minted(overflow_error(self.minted().checked_add(1u128))?);
    Ok(())
  }
  pub fn value_per_mint_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/value-per-mint")
  }
  pub fn value_per_mint(&self) -> u128 {
    self.value_per_mint_pointer().get_value::<u128>()
  }
  pub fn set_value_per_mint(&self, v: u128) {
    self.value_per_mint_pointer().set_value::<u128>(v);
  }
  pub fn cap_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/cap")
  }
  pub fn cap(&self) -> u128 {
    self.cap_pointer().get_value::<u128>()
  }
  pub fn set_cap(&self, v: u128) {
    self.cap_pointer().set_value::<u128>(if v == 0 { u128::MAX } else { v })
  }
}

impl AlkaneResponder for MintableAlkane {
    fn execute(&self) -> Result<CallResponse> {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let mut response = CallResponse::forward(&context.incoming_alkanes);
        match shift_or_err(&mut inputs)? {
          0 => {
            let token_units = shift_or_err(&mut inputs)?;
            self.set_value_per_mint(shift_or_err(&mut inputs)?);
            self.set_cap(shift_or_err(&mut inputs)?); // use 0 for an unlimited supply
            let _ = self.set_data();
            self.set_name_and_symbol(shift_or_err(&mut inputs)?, shift_or_err(&mut inputs)?);
            response.alkanes.0.push(self.mint(&context, token_units).unwrap());
            Ok(response)
          }
          77 => {
            response.alkanes.0.push(self.mint(&context, self.value_per_mint()).unwrap());
            self.increment_mint()?;
            if self.minted() > self.cap() {
              Err(anyhow!("supply has reached cap"))
            } else {
              Ok(response)
            }
          }
          99 => {
            response.data = self.name().into_bytes().to_vec();
            Ok(response)
          }
          100 => {
            response.data = self.symbol().into_bytes().to_vec();
            Ok(response)
          }
          101 => {
            response.data = self.total_supply().to_le_bytes().to_vec();
            Ok(response)
          }
          102 => {
            response.data = self.cap().to_le_bytes().to_vec();
            Ok(response)
          }
          103 => {
            response.data = self.minted().to_le_bytes().to_vec();
            Ok(response)
          }
          1000 => {
            response.data = self.data();
            Ok(response)
          }
          _ => {
            Err(anyhow!("unrecognized opcode"))
          }
        }
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&MintableAlkane::default().run());
    to_ptr(&mut response) + 4
}
