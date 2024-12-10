use alkane_factory_support::factory::{MintableToken};
use alkanes_runtime::runtime::AlkaneResponder;
use alkanes_support::{response::CallResponse, utils::shift};
use alkanes_runtime::storage::{StoragePointer};
use metashrew_support::index_pointer::{KeyValuePointer};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};

#[derive(Default)]
pub struct MintableAlkane(());


impl MintableToken for MintableAlkane {}

impl MintableAlkane {
  pub fn value_per_mint_pointer(&self) -> StoragePointer {
    StoragePointer::from_keyword("/value-per-mint")
  }
  pub fn value_per_mint(&self) -> u128 {
    self.value_per_mint_pointer().get_value::<u128>()
  }
  pub fn set_value_per_mint(&self, v: u128) {
    self.value_per_mint_pointer().set_value::<u128>(v);
  }
}

impl AlkaneResponder for MintableAlkane {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let mut response = CallResponse::forward(&context.incoming_alkanes);
        match shift(&mut inputs).unwrap() {
          0 => {
            let token_units = shift(&mut inputs).unwrap(); 
            self.set_name_and_symbol(shift(&mut inputs).unwrap(), shift(&mut inputs).unwrap());
            response.alkanes.0.push(self.mint(&context, token_units).unwrap());
            response
          }
          77 => {
            response.alkanes.0.push(self.mint(&context, self.value_per_mint()).unwrap());
            response
          }
          99 => {
            response.data = self.name().into_bytes().to_vec();
            response
          }
          100 => {
            response.data = self.symbol().into_bytes().to_vec();
            response
          }
          101 => {
            response.data = self.total_supply().to_le_bytes().to_vec();
            response
          }
          _ => {
            panic!("unrecognized opcode");
          }
        }
    }
}

#[no_mangle]
pub extern "C" fn __execute() -> i32 {
    let mut response = to_arraybuffer_layout(&MintableAlkane::default().run());
    to_ptr(&mut response) + 4
}
