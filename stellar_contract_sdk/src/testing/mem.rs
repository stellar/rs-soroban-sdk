use core::panic;

use super::MockHost;
use crate::object::*;
use crate::{status, Object, OrAbort, Val};
use im_rc::{HashMap, Vector};
use num_bigint::BigInt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Address(pub Vec<u8>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Asset {
    Native,
    Credit { code: String, issuer: Address },
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContractID(u64);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ContractKey(Address, ContractID);

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MemLedgerKey {
    Account(Address),
    TrustLine { account: Address, asset: Asset },
    ContractCode(ContractKey),
    ContractData(ContractKey, Val),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MemLedgerVal {
    Account(i64),
    Asset(Asset),
    TrustLine(i64),
    ContractData(Val),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MemOperation {}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MemOperationResult {
    Ok,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MemTransaction {}

type HostMap = HashMap<Val, Val>;
type HostVec = Vector<Val>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MemObj {
    Box(Val),
    Map(HostMap),
    Vec(HostVec),
    U64(u64),
    I64(i64),
    Str(String),
    Blob(Vec<u8>),
    LedgerKey(MemLedgerKey),
    LedgerVal(MemLedgerVal),
    Operation(MemOperation),
    OperationResult(MemOperationResult),
    Transaction(MemTransaction),
    BigNum(BigInt),
    // ...
}

impl MemObj {
    fn object_type(&self) -> u8 {
        match self {
            MemObj::Box(_) => OBJ_BOX,
            MemObj::Map(_) => OBJ_MAP,
            MemObj::Vec(_) => OBJ_VEC,
            MemObj::U64(_) => OBJ_U64,
            MemObj::I64(_) => OBJ_I64,
            MemObj::Str(_) => OBJ_STRING,
            MemObj::Blob(_) => OBJ_BINARY,
            MemObj::LedgerKey(_) => OBJ_LEDGERKEY,
            MemObj::LedgerVal(_) => OBJ_LEDGERVAL,
            MemObj::Operation(_) => OBJ_OPERATION,
            MemObj::OperationResult(_) => OBJ_OPERATION_RESULT,
            MemObj::Transaction(_) => OBJ_TRANSACTION,
            MemObj::BigNum(_) => OBJ_BIGNUM,
        }
    }
}

pub struct MemHost {
    context: Vec<ContractKey>,
    objs: Vec<MemObj>,
    ledger: HashMap<MemLedgerKey, MemLedgerVal>,
    last_op_result: MemOperationResult,
}

impl MemHost {
    fn current_contract_data_key(&self, k: Val) -> MemLedgerKey {
        MemLedgerKey::ContractData(self.context.last().expect("missing context").clone(), k)
    }

    fn read_map_with<F, T>(&self, m: Object, f: F) -> T
    where
        F: FnOnce(&HostMap) -> T,
    {
        match self.get_obj(m).expect("missing map object") {
            MemObj::Map(mm) => f(mm),
            _ => panic!("wrong object type"),
        }
    }

    fn new_map_with<F>(&mut self, m: Object, f: F) -> Object
    where
        F: FnOnce(&HostMap) -> HostMap,
    {
        let newobj = match self.get_obj(m).expect("missing map object") {
            MemObj::Map(mm) => f(mm),
            _ => panic!("wrong object type"),
        };
        self.put_obj(MemObj::Map(newobj))
    }

    fn read_vec_with<F, T>(&self, m: Object, f: F) -> T
    where
        F: FnOnce(&HostVec) -> T,
    {
        match self.get_obj(m).expect("missing vec object") {
            MemObj::Vec(vv) => f(vv),
            _ => panic!("wrong object type"),
        }
    }

    fn new_vec_with<F>(&mut self, m: Object, f: F) -> Object
    where
        F: FnOnce(&HostVec) -> HostVec,
    {
        let newobj = match self.get_obj(m).expect("missing vec object") {
            MemObj::Vec(vv) => f(vv),
            _ => panic!("wrong object type"),
        };
        self.put_obj(MemObj::Vec(newobj))
    }
}

impl MockHost for MemHost {
    fn log_value(&mut self, v: Val) -> Val {
        eprintln!("MemHost::log_value: {:?}", v);
        Val::from_void()
    }

    fn get_last_operation_result(&mut self) -> Object {
        let ob = MemObj::OperationResult(self.last_op_result.clone());
        self.put_obj(ob)
    }

    fn u64_from_u64(&mut self, u: u64) -> Object {
        let ob = MemObj::U64(u);
        self.put_obj(ob)
    }

    fn u64_to_u64(&mut self, u: Object) -> u64 {
        match self.get_obj(u).expect("missing object") {
            MemObj::U64(v) => *v,
            _ => panic!("wrong object type"),
        }
    }

    fn i64_from_i64(&mut self, i: i64) -> Object {
        let ob = MemObj::I64(i);
        self.put_obj(ob)
    }

    fn i64_to_i64(&mut self, i: Object) -> i64 {
        match self.get_obj(i).expect("missing object") {
            MemObj::I64(v) => *v,
            _ => panic!("wrong object type"),
        }
    }

    fn map_new(&mut self) -> Object {
        let ob = MemObj::Map(HashMap::new());
        self.put_obj(ob)
    }

    fn map_put(&mut self, m: Object, k: Val, v: Val) -> Object {
        self.new_map_with(m, |mm| mm.update(k, v))
    }

    fn map_get(&mut self, m: Object, k: Val) -> Val {
        self.read_map_with(m, |mm| mm.get(&k).expect("missing map entry").clone())
    }

    fn map_del(&mut self, m: Object, k: Val) -> Object {
        self.new_map_with(m, |mm| mm.without(&k))
    }

    fn map_len(&mut self, m: Object) -> Val {
        let sz = self.read_map_with(m, |mm| mm.len());
        assert!(sz as u64 <= u32::MAX as u64);
        (sz as u32).into()
    }

    fn map_keys(&mut self, m: Object) -> Object {
        let vv: HostVec = self.read_map_with(m, |mm| mm.keys().cloned().collect());
        self.put_obj(MemObj::Vec(vv))
    }

    fn map_has(&mut self, m: Object, k: Val) -> Val {
        self.read_map_with(m, |mm| mm.contains_key(&k).into())
    }

    fn pay(&mut self, src: Object, dst: Object, asset: Object, amount: Val) -> Val {
        let src_addr = match self.get_obj(src).expect("missing account") {
            MemObj::LedgerKey(MemLedgerKey::Account(a)) => a.clone(),
            _ => panic!("wrong object type"),
        };
        let dst_addr = match self.get_obj(dst).expect("missing account") {
            MemObj::LedgerKey(MemLedgerKey::Account(a)) => a.clone(),
            _ => panic!("wrong object type"),
        };
        let (asset_code, asset_issuer) = match self.get_obj(asset).expect("missing asset") {
            MemObj::LedgerVal(MemLedgerVal::Asset(Asset::Credit { code, issuer })) => {
                (code.clone(), issuer.clone())
            }
            MemObj::LedgerVal(MemLedgerVal::Asset(_)) => todo!(),
            _ => panic!("wrong object type"),
        };
        let asset = Asset::Credit {
            code: asset_code.clone(),
            issuer: asset_issuer.clone(),
        };
        let amount: i64 = amount.try_into().or_abort();

        let src_tlk = MemLedgerKey::TrustLine {
            account: src_addr.clone(),
            asset: asset.clone(),
        };
        let dst_tlk = MemLedgerKey::TrustLine {
            account: dst_addr.clone(),
            asset: asset.clone(),
        };

        let src_bal = match self.get_ledger_value(src_tlk.clone()) {
            Some(MemLedgerVal::TrustLine(b)) => Some(b),
            None => {
                if src_addr == asset_issuer {
                    None
                } else {
                    panic!("src does not have trust line")
                }
            }
            _ => panic!("src wrong ledger entry type"),
        };
        let dst_bal = match self.get_ledger_value(dst_tlk.clone()) {
            Some(MemLedgerVal::TrustLine(b)) => Some(b),
            None => {
                if dst_addr == asset_issuer {
                    None
                } else {
                    panic!("dst does not have trust line")
                }
            }
            _ => panic!("dst wrong ledger entry type"),
        };

        if let Some(src_bal) = src_bal {
            assert!(src_bal >= amount, "src balance insufficient");
            let src_new_bal = src_bal - amount;
            self.put_ledger_value(src_tlk.clone(), MemLedgerVal::TrustLine(src_new_bal));
        }

        if let Some(dst_bal) = dst_bal {
            let dst_new_bal = dst_bal.checked_add(amount).expect("dst balance overflow");
            self.put_ledger_value(dst_tlk.clone(), MemLedgerVal::TrustLine(dst_new_bal));
        }

        status::OK.into()
    }

    fn account_balance(&mut self, acc: Object) -> Val {
        todo!()
    }

    fn account_trust_line(&mut self, acc: Object, asset: Object) -> Object {
        let acc_addr = match self.get_obj(acc).expect("missing account") {
            MemObj::LedgerKey(MemLedgerKey::Account(addr)) => addr.clone(),
            _ => panic!("wrong object type"),
        };
        let asset = match self.get_obj(asset).expect("missing asset") {
            MemObj::LedgerVal(MemLedgerVal::Asset(asset)) => asset.clone(),
            _ => panic!("wrong object type"),
        };
        self.put_obj(MemObj::LedgerKey(MemLedgerKey::TrustLine {
            account: acc_addr,
            asset: asset,
        }))
    }

    fn trust_line_balance(&mut self, tl: Object) -> Val {
        let lk = match self.get_obj(tl).expect("missing trust line") {
            MemObj::LedgerKey(lk) => lk.clone(),
            _ => panic!("wrong object type"),
        };
        let bal = match self.ledger.get(&lk).expect("missing ledger key") {
            MemLedgerVal::TrustLine(bal) => *bal,
            _ => panic!("wrong ledger key type"),
        };
        bal.try_into().or_abort()
    }

    fn get_contract_data(&mut self, k: Val) -> Val {
        let lk = self.current_contract_data_key(k);
        match self.ledger.get(&lk).expect("missing ledger key") {
            MemLedgerVal::ContractData(v) => v.clone(),
            _ => panic!("wrong ledger key type"),
        }
    }

    fn put_contract_data(&mut self, k: Val, v: Val) -> Val {
        let lk = self.current_contract_data_key(k);
        self.ledger.insert(lk, MemLedgerVal::ContractData(v));
        Val::from_void()
    }

    fn has_contract_data(&mut self, k: Val) -> Val {
        let lk = self.current_contract_data_key(k);
        self.ledger.contains_key(&lk).into()
    }

    fn vec_new(&mut self) -> Object {
        let ob = MemObj::Vec(Vector::new());
        self.put_obj(ob)
    }

    fn vec_put(&mut self, v: Object, i: Val, x: Val) -> Object {
        let idx = i.as_u32() as usize;
        self.new_vec_with(v, |vv| vv.update(idx, x))
    }

    fn vec_get(&mut self, v: Object, i: Val) -> Val {
        todo!()
    }

    fn vec_del(&mut self, v: Object, i: Val) -> Object {
        let idx = i.as_u32() as usize;
        self.new_vec_with(v, |vv| {
            let mut vv = vv.clone();
            vv.remove(idx);
            vv
        })
    }

    fn vec_len(&mut self, v: Object) -> Val {
        let sz = self.read_vec_with(v, |vv| vv.len());
        assert!(sz as u64 <= u32::MAX as u64);
        (sz as u32).into()
    }

    fn vec_push(&mut self, v: Object, x: Val) -> Object {
        self.new_vec_with(v, |vv| {
            let mut vv = vv.clone();
            vv.push_back(x);
            vv
        })
    }

    fn vec_pop(&mut self, v: Object) -> Object {
        self.new_vec_with(v, |vv| {
            let mut vv = vv.clone();
            vv.pop_back();
            vv
        })
    }

    fn vec_take(&mut self, v: Object, n: Val) -> Object {
        let n = n.as_u32() as usize;
        self.new_vec_with(v, |vv| vv.take(n))
    }

    fn vec_drop(&mut self, v: Object, n: Val) -> Object {
        let n = n.as_u32() as usize;
        self.new_vec_with(v, |vv| vv.skip(n))
    }

    fn vec_front(&mut self, v: Object) -> Val {
        self.read_vec_with(v, |vv| vv.front().expect("front on empty vector").clone())
    }

    fn vec_back(&mut self, v: Object) -> Val {
        self.read_vec_with(v, |vv| vv.back().expect("back on empty vector").clone())
    }

    fn vec_insert(&mut self, v: Object, i: Val, x: Val) -> Object {
        let idx = i.as_u32() as usize;
        self.new_vec_with(v, |vv| {
            let mut vv = vv.clone();
            vv.insert(idx, x);
            vv
        })
    }

    fn vec_append(&mut self, v1: Object, v2: Object) -> Object {
        let v2 = self.read_vec_with(v2, |vv| vv.clone());
        self.new_vec_with(v1, |vv| {
            let mut vv = vv.clone();
            vv.append(v2);
            vv
        })
    }
}

impl MemHost {
    pub fn new() -> Self {
        let contract = ContractKey(Address(Vec::new()), ContractID(0));
        let mut context = Vec::new();
        context.push(contract);
        let objs = Vec::new();
        let last_op_result = MemOperationResult::Ok;
        let ledger = HashMap::new();
        Self {
            context,
            objs,
            last_op_result,
            ledger,
        }
    }

    pub fn put_obj(&mut self, ob: MemObj) -> Object {
        let idx = self.objs.len();
        let ty = ob.object_type();
        self.objs.push(ob);
        Object::from_type_and_idx(ty, idx)
    }

    pub fn get_obj(&self, ob: Object) -> Option<&MemObj> {
        let idx = ob.get_idx();
        if idx < self.objs.len() {
            Some(&self.objs[idx])
        } else {
            None
        }
    }

    pub fn put_ledger_value(&mut self, k: MemLedgerKey, v: MemLedgerVal) {
        self.ledger.insert(k, v);
    }

    pub fn get_ledger_value(&mut self, k: MemLedgerKey) -> Option<MemLedgerVal> {
        self.ledger.get(&k).map(|v| v.clone())
    }
}
