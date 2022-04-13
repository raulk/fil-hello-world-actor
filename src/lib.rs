mod blockstore;

use crate::blockstore::Blockstore;
use cid::multihash::Code;
use cid::Cid;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};
use fvm_ipld_encoding::{to_vec, CborStore, RawBytes, DAG_CBOR};
use fvm_sdk as sdk;
use fvm_sdk::message::NO_DATA_BLOCK_ID;

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct State {
    pub count: u64,
}

macro_rules! abort {
    ($code:ident, $msg:literal $(, $ex:expr)*) => {
        fvm_sdk::vm::abort(
            fvm_shared::error::ExitCode::$code.value(),
            Some(format!($msg, $($ex,)*).as_str()),
        )
    };
}

/// The actor's WASM entrypoint. It takes the ID of the parameters block,
/// and returns the ID of the return value block, or NO_DATA_BLOCK_ID if no
/// return value.
#[no_mangle]
pub fn invoke(_: u32) -> u32 {
    match sdk::message::method_number() {
        1 => {
            constructor();
            return NO_DATA_BLOCK_ID;
        }
        2 => {} // fallthrough
        _ => {
            abort!(USR_UNHANDLED_MESSAGE, "unrecognized method");
        }
    }

    // First, load the current state root.
    let root = match sdk::sself::root() {
        Ok(root) => root,
        Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get root: {:?}", err),
    };

    // Load the actor state from the state tree.
    let state = match Blockstore.get_cbor::<State>(&root) {
        Ok(Some(state)) => state,
        Ok(None) => abort!(USR_ILLEGAL_STATE, "state does not exist"),
        Err(err) => abort!(USR_ILLEGAL_STATE, "failed to get state: {}", err),
    };

    // Conduct method dispatch. Handle input parameters and return data.
    let ret: Option<RawBytes> = say_hello(state);

    // Insert the return data block if necessary, and return the correct
    // block ID.
    match ret {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        },
    }
}

pub fn constructor() -> Option<RawBytes> {
    let state = State::default();
    save_state(&state);
    None
}

fn save_state(state: &State) -> Cid {
    let serialized = match to_vec(&state) {
        Ok(s) => s,
        Err(err) => abort!(
            USR_SERIALIZATION,
            "failed to serialize initial state: {:?}",
            err
        ),
    };
    let cid = match sdk::ipld::put(Code::Blake2b256.into(), 32, DAG_CBOR, serialized.as_slice()) {
        Ok(cid) => cid,
        Err(err) => abort!(USR_SERIALIZATION, "failed to store initial state: {:}", err),
    };
    if let Err(err) = sdk::sself::set_root(&cid) {
        abort!(USR_ILLEGAL_STATE, "failed to set root ciid: {:}", err);
    }
    cid
}

pub fn say_hello(mut state: State) -> Option<RawBytes> {
    state.count += 1;
    save_state(&state);

    let ret = to_vec(format!("Hello world #{}!", &state.count).as_str());
    match ret {
        Ok(ret) => Some(RawBytes::new(ret)),
        Err(err) => {
            abort!(
                USR_ILLEGAL_STATE,
                "failed to serialize return value: {:?}",
                err
            );
        }
    }
}
