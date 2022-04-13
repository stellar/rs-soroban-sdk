use super::MockHost;
use crate::{Object, Val};
use im_rc::{HashMap, Vector};
use num_bigint::BigInt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Address(pub Vec<u8>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Asset {
    pub code: String,
    pub issuer: Address,
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

    fn pay(&mut self, src: Val, dst: Val, asset: Val, amount: Val) -> Val {
        todo!()
    }

    fn account_balance(&mut self, acc: Val, asset: Val) -> Val {
        todo!()
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
        self.objs.push(ob);
        Object::from_idx(idx)
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
