use econf::LoadEnv;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::path::PathBuf;

#[derive(LoadEnv)]
struct Boolean {
    a: bool,
    b: bool,
}

#[test]
fn boolean() {
    std::env::set_var("BOOL_B", "true");

    let a = Boolean { a: false, b: false };
    let a = econf::load(a, "bool");
    assert_eq!(a.a, false);
    assert_eq!(a.b, true);
}

#[derive(LoadEnv)]
struct Numbers {
    f1: f32,
    f2: f64,
    sz: isize,
    i1: i8,
    i2: i16,
    i3: i32,
    i4: i64,
    usz: usize,
    u1: u8,
    u2: u16,
    u3: u32,
    u4: u64,
}

#[test]
fn numbers() {
    std::env::set_var("NUMBERS_F2", "32.3");
    std::env::set_var("NUMBERS_SZ", "122233");
    std::env::set_var("NUMBERS_I3", "-32393");
    std::env::set_var("NUMBERS_U4", "1384279284");

    let a = Numbers {
        f1: 1.2,
        f2: 3.1,
        sz: 3267849,
        i1: -39,
        i2: -100,
        i3: 322,
        i4: 32897323,
        usz: 3247683283,
        u1: 39,
        u2: 328,
        u3: 311900,
        u4: 36718928,
    };
    let a = econf::load(a, "numbers");
    assert_eq!(a.f1, 1.2);
    assert_eq!(a.f2, 32.3);
    assert_eq!(a.sz, 122233);
    assert_eq!(a.i1, -39);
    assert_eq!(a.i2, -100);
    assert_eq!(a.i3, -32393);
    assert_eq!(a.i4, 32897323);
    assert_eq!(a.usz, 3247683283);
    assert_eq!(a.u1, 39);
    assert_eq!(a.u2, 328);
    assert_eq!(a.u3, 311900);
    assert_eq!(a.u4, 1384279284);
}

#[derive(LoadEnv)]
struct Seqs {
    f1: Vec<f32>,
    f2: Vec<f64>,
    sz: Vec<isize>,
    i1: Vec<i8>,
    i2: Vec<i64>,
    usz: Vec<usize>,
    u1: Vec<u8>,
    u2: Vec<u64>,
    l1: LinkedList<bool>,
    l2: VecDeque<u32>,
    l3: BinaryHeap<String>,
    s1: HashSet<String>,
    s2: HashSet<char>,
    s3: BTreeSet<String>,
}

#[test]
fn vectors() {
    std::env::set_var("SEQS_F1", "[1.1, 1.2, 1.3]");
    std::env::set_var("SEQS_USZ", "[]");
    std::env::set_var("SEQS_I1", "[-1, -3, 4, 5, 6]");
    std::env::set_var("SEQS_U2", "[131239, 1930421, 115]");
    std::env::set_var("SEQS_L1", "[true, true, false, true, false]");
    std::env::set_var("SEQS_L2", "[3, 9,3, 9]");
    std::env::set_var("SEQS_L3", "[Z, X, Y]");
    std::env::set_var("SEQS_S2", "[p, q, r, p]");
    std::env::set_var("SEQS_S3", "[PP, QQ, RR, QQ, RR]");

    let a = Seqs {
        f1: vec![1.2, 33.0],
        f2: vec![1.1],
        sz: vec![1, 3, 41, 55],
        i1: vec![-3, 23],
        i2: vec![-1, 32, -100],
        usz: vec![323, 424],
        u1: vec![1, 1, 1],
        u2: vec![1, 2, 3],
        l1: LinkedList::default(),
        l2: VecDeque::default(),
        l3: BinaryHeap::default(),
        s1: HashSet::new(),
        s2: HashSet::new(),
        s3: BTreeSet::new(),
    };
    let a = econf::load(a, "seqs");
    assert_eq!(a.f1, vec![1.1, 1.2, 1.3]);
    assert_eq!(a.f2, vec![1.1]);
    assert_eq!(a.sz, vec![1, 3, 41, 55]);
    assert_eq!(a.i1, vec![-1, -3, 4, 5, 6]);
    assert_eq!(a.i2, vec![-1, 32, -100]);
    assert_eq!(a.usz, Vec::<usize>::new());
    assert_eq!(a.u1, vec![1, 1, 1]);
    assert_eq!(a.u2, vec![131239, 1930421, 115]);
    let mut l1 = LinkedList::new();
    l1.push_back(true);
    l1.push_back(true);
    l1.push_back(false);
    l1.push_back(true);
    l1.push_back(false);
    assert_eq!(a.l1, l1);
    let mut l2 = VecDeque::new();
    l2.push_back(3);
    l2.push_back(9);
    l2.push_back(3);
    l2.push_back(9);
    assert_eq!(a.l2, l2);
    let mut l3 = BinaryHeap::new();
    l3.push("X".into());
    l3.push("Y".into());
    l3.push("Z".into());
    let mut l3orig = a.l3.clone();
    (0..3).for_each(|_| {
        assert_eq!(l3.pop(), l3orig.pop());
    });
    assert_eq!(a.s1, HashSet::new());
    let mut s2 = HashSet::new();
    s2.insert('p');
    s2.insert('q');
    s2.insert('r');
    assert_eq!(a.s2, s2);
    let mut s3 = BTreeSet::new();
    s3.insert("RR".into());
    s3.insert("RR".into());
    s3.insert("PP".into());
    s3.insert("QQ".into());
    s3.insert("QQ".into());
    assert_eq!(a.s3, s3);
}

#[derive(Default, LoadEnv)]
struct Maps {
    m1: HashMap<i8, u32>,
    m2: HashMap<i64, String>,
    m3: HashMap<u8, u32>,
    m4: HashMap<u64, String>,
    m5: BTreeMap<usize, String>,
    m6: BTreeMap<i32, String>,
}

#[test]
fn maps() {
    std::env::set_var("MAPS_M1", "{9: 3, 1: 2, 4: 8}");
    std::env::set_var("MAPS_M2", "{-1: gomi, -3: kami, 9: 5}");
    std::env::set_var("MAPS_M6", "{-1: gomi, -3: kami, 9: 5}");

    let a = Maps::default();
    let a = econf::load(a, "maps");
    let mut m1 = HashMap::new();
    m1.insert(9, 3);
    m1.insert(1, 2);
    m1.insert(4, 8);
    assert_eq!(a.m1, m1);
    let mut m2 = HashMap::new();
    m2.insert(-1, "gomi".into());
    m2.insert(-3, "kami".into());
    m2.insert(9, "5".into());
    assert_eq!(a.m2, m2);
    assert_eq!(a.m3, HashMap::default());
    assert_eq!(a.m4, HashMap::default());
    assert_eq!(a.m5, BTreeMap::default());
    let mut m6 = BTreeMap::new();
    m6.insert(-1, "gomi".into());
    m6.insert(-3, "kami".into());
    m6.insert(9, "5".into());
    assert_eq!(a.m6, m6);
}

#[derive(LoadEnv)]
struct Chars {
    s1: String,
    s2: String,
    s3: String,
    s4: char,
    s5: char,
}

#[test]
fn chars() {
    std::env::set_var("CHARS_S1", "Hello World");
    std::env::set_var("CHARS_S3", "[1,2,3]");
    std::env::set_var("CHARS_S5", "D");

    let a = Chars {
        s1: "Gomi".into(),
        s2: "Kami".into(),
        s3: "Semi".into(),
        s4: 'p',
        s5: 'q',
    };
    let a = econf::load(a, "chars");
    assert_eq!(a.s1, "Hello World");
    assert_eq!(a.s2, "Kami");
    assert_eq!(a.s3, "[1,2,3]");
    assert_eq!(a.s4, 'p');
    assert_eq!(a.s5, 'D');
}

#[derive(LoadEnv)]
struct Nested {
    chs: Chars,
    map: Maps,
}

#[test]
fn nested() {
    std::env::set_var("NESTED_CHS_S2", "__init__");
    std::env::set_var("NESTED_MAP_M4", "{1: gay, 2: straight}");

    let a = Nested {
        chs: Chars {
            s1: "Gomi".into(),
            s2: "Kami".into(),
            s3: "Semi".into(),
            s4: 'p',
            s5: 'q',
        },
        map: Maps::default(),
    };
    let a = econf::load(a, "nested");
    assert_eq!(a.chs.s1, "Gomi");
    assert_eq!(a.chs.s2, "__init__");
    assert_eq!(a.chs.s3, "Semi");
    assert_eq!(a.chs.s4, 'p');
    assert_eq!(a.chs.s5, 'q');
    assert_eq!(a.map.m1, HashMap::default());
    assert_eq!(a.map.m2, HashMap::default());
    assert_eq!(a.map.m3, HashMap::default());
    let mut m4 = HashMap::new();
    m4.insert(1, "gay".into());
    m4.insert(2, "straight".into());
    assert_eq!(a.map.m4, m4);
    assert_eq!(a.map.m5, BTreeMap::default());
    assert_eq!(a.map.m6, BTreeMap::default());
}

#[derive(LoadEnv)]
struct Tuples {
    t1: (String, u32, char),
    t2: (bool, i8, String),
    t3: (Vec<u32>, f32),
}

#[test]
fn tuples() {
    std::env::set_var("TUPLES_T2", "[false, -11, gaybar]");
    std::env::set_var("TUPLES_T3", "[[9,8,8], -99.9]");

    let a = Tuples {
        t1: ("GB".into(), 39, 'c'),
        t2: (true, -98, "SB".into()),
        t3: (vec![1, 3, 3], -43.2),
    };
    let a = econf::load(a, "tuples");
    assert_eq!(a.t1, ("GB".into(), 39, 'c'));
    assert_eq!(a.t2, (false, -11, "gaybar".into()));
    assert_eq!(a.t3, (vec![9, 8, 8], -99.9));
}

#[derive(LoadEnv, PartialEq, Debug)]
struct TS1(String, u32, char);

#[derive(LoadEnv, PartialEq, Debug)]
struct TS2(bool, i8, String);

#[derive(LoadEnv, PartialEq, Debug)]
struct TS3(Vec<u32>, f32);

#[derive(LoadEnv)]
struct TupleStruct {
    t1: TS1,
    t2: TS2,
    t3: TS3,
}

#[test]
fn tuple_struct() {
    std::env::set_var("TUPLE_STRUCT_T2_2", "gaybar");
    std::env::set_var("TUPLE_STRUCT_T3_0", "[11,11,12]");

    let a = TupleStruct {
        t1: TS1("GB".into(), 39, 'c'),
        t2: TS2(true, -98, "SB".into()),
        t3: TS3(vec![1, 3, 3], -43.2),
    };
    let a = econf::load(a, "tuple_struct");
    assert_eq!(a.t1, TS1("GB".into(), 39, 'c'));
    assert_eq!(a.t2, TS2(true, -98, "gaybar".into()));
    assert_eq!(a.t3, TS3(vec![11, 11, 12], -43.2));
}

struct NotLoadEnv {
    s: String,
}

#[derive(LoadEnv)]
struct Skipped {
    v1: bool,
    #[econf(skip)]
    v2: u32,
    #[econf(skip)]
    v3: NotLoadEnv,
}

#[test]
fn skipped() {
    std::env::set_var("SKIPPED_V1", "true");
    std::env::set_var("SKIPPED_V2", "0");
    std::env::set_var("SKIPPED_V3", "skipped");

    let a = Skipped {
        v1: false,
        v2: 42,
        v3: NotLoadEnv {
            s: "initial".to_string(),
        },
    };

    let a = econf::load(a, "skipped");
    assert_eq!(a.v1, true);
    assert_eq!(a.v2, 42);
    assert_eq!(a.v3.s, "initial".to_string());
}

#[derive(LoadEnv)]
struct Net {
    n1: IpAddr,
    n2: Ipv4Addr,
    n3: Ipv6Addr,
    n4: SocketAddr,
    n5: SocketAddrV4,
    n6: SocketAddrV6,
}

#[test]
fn net() {
    std::env::set_var("NET_N2", "127.0.0.1");
    std::env::set_var("NET_N4", "127.0.0.1:9999");
    std::env::set_var("NET_N6", "[2001:db8:85a3:8d3:1319:8a2e:370:7348]:9898");

    let a = Net {
        n1: "192.168.0.1".parse().unwrap(),
        n2: "192.168.0.1".parse().unwrap(),
        n3: "::1".parse().unwrap(),
        n4: "192.168.0.1:8080".parse().unwrap(),
        n5: "192.168.0.1:8080".parse().unwrap(),
        n6: "[2001:db8::1]:8080".parse().unwrap(),
    };
    let a = econf::load(a, "net");
    assert_eq!(Ok(a.n1), "192.168.0.1".parse());
    assert_eq!(Ok(a.n2), "127.0.0.1".parse());
    assert_eq!(Ok(a.n3), "::1".parse());
    assert_eq!(Ok(a.n4), "127.0.0.1:9999".parse());
    assert_eq!(Ok(a.n5), "192.168.0.1:8080".parse());
    assert_eq!(
        Ok(a.n6),
        "[2001:db8:85a3:8d3:1319:8a2e:370:7348]:9898".parse()
    );
}

#[derive(LoadEnv)]
struct Options {
    o1: Option<u32>,
    o2: Option<i32>,
    o3: Option<String>,
    o4: Option<String>,
}

#[test]
fn options() {
    std::env::set_var("OPTIONS_O1", "~");
    std::env::set_var("OPTIONS_O4", "Hage");

    let a = Options {
        o1: Some(9),
        o2: None,
        o3: Some("gomi".into()),
        o4: None,
    };
    let a = econf::load(a, "options");
    assert_eq!(a.o1, None);
    assert_eq!(a.o2, None);
    assert_eq!(a.o3, Some("gomi".into()));
    assert_eq!(a.o4, Some("Hage".into()));
}

#[derive(LoadEnv)]
#[allow(non_snake_case)]
struct Capital {
    VALUE: i32,
}

#[test]
#[allow(non_snake_case)]
fn capital() {
    std::env::set_var("CAPITAL_VALUE", "10");

    let a = Capital { VALUE: 0 };
    let a = econf::load(a, "capital");
    assert_eq!(a.VALUE, 10);
}

use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

#[derive(LoadEnv)]
struct NonZeroNumbers {
    sz: NonZeroIsize,
    i1: NonZeroI8,
    i2: NonZeroI16,
    i3: NonZeroI32,
    i4: NonZeroI64,
    i5: NonZeroI128,
    usz: NonZeroUsize,
    u1: NonZeroU8,
    u2: NonZeroU16,
    u3: NonZeroU32,
    u4: NonZeroU64,
    u5: NonZeroU128,
}

#[test]
fn non_zero_numbers() {
    simple_logger::init().unwrap();

    std::env::set_var("NZNUMBERS_SZ", "122233");
    std::env::set_var("NZNUMBERS_U4", "1384279284");
    std::env::set_var("NZNUMBERS_I3", "-32393");
    std::env::set_var("NZNUMBERS_I5", "-1111384279284");

    std::env::set_var("NZNUMBERS_U1", "0"); // Results in error log
    std::env::set_var("NZNUMBERS_I2", "0"); // Results in error log

    let a = NonZeroNumbers {
        sz: NonZeroIsize::new(3267849).unwrap(),
        i1: NonZeroI8::new(-39).unwrap(),
        i2: NonZeroI16::new(-100).unwrap(),
        i3: NonZeroI32::new(322).unwrap(),
        i4: NonZeroI64::new(32897323).unwrap(),
        i5: NonZeroI128::new(32897323).unwrap(),
        usz: NonZeroUsize::new(3247683283).unwrap(),
        u1: NonZeroU8::new(39).unwrap(),
        u2: NonZeroU16::new(328).unwrap(),
        u3: NonZeroU32::new(311900).unwrap(),
        u4: NonZeroU64::new(36718928).unwrap(),
        u5: NonZeroU128::new(111132897323).unwrap(),
    };

    let a = econf::load(a, "nznumbers");
    assert_eq!(a.sz.get(), 122233);
    assert_eq!(a.i1.get(), -39);
    assert_eq!(a.i2.get(), -100);
    assert_eq!(a.i3.get(), -32393);
    assert_eq!(a.i4.get(), 32897323);
    assert_eq!(a.i5.get(), -1111384279284);
    assert_eq!(a.usz.get(), 3247683283);
    assert_eq!(a.u1.get(), 39);
    assert_eq!(a.u2.get(), 328);
    assert_eq!(a.u3.get(), 311900);
    assert_eq!(a.u4.get(), 1384279284);
    assert_eq!(a.u5.get(), 111132897323);
}

use std::time::Duration;

#[derive(LoadEnv)]
struct Durations {
    d1: Duration,
    d2: Duration,
    d3: Duration,
}

#[test]
fn duration() {
    std::env::set_var("DURATIONS_D2", "1m");
    std::env::set_var("DURATIONS_D3", "1h");

    let a = Durations {
        d1: Duration::from_secs(100),
        d2: Duration::from_secs(1000),
        d3: Duration::from_secs(1),
    };

    let a = econf::load(a, "durations");
    assert_eq!(a.d1, Duration::from_secs(100));
    assert_eq!(a.d2, Duration::from_secs(60));
    assert_eq!(a.d3, Duration::from_secs(3600));
}

#[derive(LoadEnv)]
struct Paths {
    p1: PathBuf,
    p2: PathBuf,
    p3: PathBuf,
}

#[test]
fn path_buf() {
    std::env::set_var("PATHS_P2", "other/path/to/file.toml");
    std::env::set_var("PATHS_P3", "data.db");

    let a = Paths {
        p1: "path/to/dir/".parse().unwrap(),
        p2: "path/to/file.toml".parse().unwrap(),
        p3: "file.rs".parse().unwrap(),
    };

    let a = econf::load(a, "paths");
    assert_eq!(a.p1, "path/to/dir/".parse::<PathBuf>().unwrap());
    assert_eq!(a.p2, "other/path/to/file.toml".parse::<PathBuf>().unwrap());
    assert_eq!(a.p3, "data.db".parse::<PathBuf>().unwrap());
}

#[test]
fn generics() {
    std::env::set_var("GENERICS_A", "33");

    #[derive(LoadEnv)]
    struct G<T>
    where
        T: LoadEnv,
    {
        a: T,
        b: String,
    }

    let g = G {
        a: 3u32,
        b: "akeome".into(),
    };

    let g = econf::load(g, "generics");
    assert_eq!(g.a, 33);
    assert_eq!(g.b, "akeome".to_string());
}
