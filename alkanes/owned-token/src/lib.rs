use alkanes_runtime::{auth::AuthenticatedResponder};
#[allow(unused_imports)]
use alkanes_runtime::{
    println,
    stdio::{stdout, Write},
};
use alkanes_runtime::{runtime::AlkaneResponder};
use alkanes_support::utils::shift;
use alkanes_support::{parcel::AlkaneTransfer, response::CallResponse};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};

use alkane_factory_support::factory::{MintableToken};

#[derive(Default)]
pub struct OwnedToken(());

impl MintableToken for OwnedToken {}

impl AuthenticatedResponder for OwnedToken {}

impl AlkaneResponder for OwnedToken {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let mut response: CallResponse = CallResponse::forward(&context.incoming_alkanes.clone());
        match shift(&mut inputs).unwrap() {
            0 => {
                self.observe_initialization().unwrap();
                let _ = self.set_data();
                let auth_token_units = shift(&mut inputs).unwrap();
                let token_units = shift(&mut inputs).unwrap();
                self.set_name_and_symbol(shift(&mut inputs).unwrap(), shift(&mut inputs).unwrap());
                response
                  .alkanes
                  .0
                  .push(self.deploy_auth_token(auth_token_units).unwrap());
                response.alkanes.0.push(AlkaneTransfer {
                  id: context.myself.clone(),
                  value: token_units,
                });
                response
            }
            77 => {
                self.only_owner().unwrap();
                let token_units = shift(&mut inputs).unwrap();
                let transfer = self.mint(&context, token_units).unwrap();
                response.alkanes.0.push(transfer);
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
            1000 => {
                response.data = self.data();
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
    let mut response = to_arraybuffer_layout(&OwnedToken::default().run());
    to_ptr(&mut response) + 4
}
