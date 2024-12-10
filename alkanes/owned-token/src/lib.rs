use alkanes_runtime::{auth::AuthenticatedResponder, token::Token};
#[allow(unused_imports)]
use alkanes_runtime::{
    println,
    stdio::{stdout, Write},
};
use alkanes_runtime::{runtime::AlkaneResponder, storage::StoragePointer};
use alkanes_support::utils::shift;
use alkanes_support::{context::Context, parcel::AlkaneTransfer, response::CallResponse};
use metashrew_support::compat::{to_arraybuffer_layout, to_ptr};
use metashrew_support::index_pointer::KeyValuePointer;
use std::sync::Arc;

#[derive(Default)]
pub struct OwnedToken(());

pub trait MintableToken: Token {
    fn mint(&self, context: &Context, value: u128) -> AlkaneTransfer {
        AlkaneTransfer {
            id: context.myself.clone(),
            value,
        }
    }
}

impl Token for OwnedToken {
    fn name(&self) -> String {
      String::from_utf8(name_pointer().get().as_ref().clone()).expect("name not saved as utf-8, did this deployment revert?")
    }
    fn symbol(&self) -> String {
      String::from_utf8(symbol_pointer().get().as_ref().clone()).expect("symbol not saved as utf-8, did this deployment revert?")
    }
}

fn trim(v: u128) -> String {
  String::from_utf8(v.to_le_bytes().into_iter().fold(Vec::<u8>::new(), |mut r, v| {
    if v != 0 {
      r.push(v)
    }
    r
  })).unwrap()
}

fn name_pointer() -> StoragePointer {
  StoragePointer::from_keyword("/name")
}

fn symbol_pointer() -> StoragePointer {
  StoragePointer::from_keyword("/symbol")
}

impl MintableToken for OwnedToken {}

impl AuthenticatedResponder for OwnedToken {}

impl AlkaneResponder for OwnedToken {
    fn execute(&self) -> CallResponse {
        let context = self.context().unwrap();
        let mut inputs = context.inputs.clone();
        let mut response: CallResponse = CallResponse::forward(&context.incoming_alkanes.clone());
        match shift(&mut inputs).unwrap() {
            0 => {
                let mut pointer = StoragePointer::from_keyword("/initialized");
                if pointer.get().len() == 0 {
                    let auth_token_units = shift(&mut inputs).unwrap();
                    let token_units = shift(&mut inputs).unwrap();
                    name_pointer().set(Arc::<Vec<u8>>::new(trim(shift(&mut inputs).unwrap()).as_bytes().to_vec()));
                    symbol_pointer().set(Arc::<Vec<u8>>::new(trim(shift(&mut inputs).unwrap()).as_bytes().to_vec()));
                    response
                        .alkanes
                        .0
                        .push(self.deploy_auth_token(auth_token_units).unwrap());
                    response.alkanes.0.push(AlkaneTransfer {
                        id: context.myself.clone(),
                        value: token_units,
                    });

                    pointer.set(Arc::new(vec![0x01]));
                    response
                } else {
                    panic!("already initialized");
                }
            }
            1 => {
                self.only_owner().unwrap();
                let token_units = shift(&mut inputs).unwrap();
                let transfer = self.mint(&context, token_units);
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
