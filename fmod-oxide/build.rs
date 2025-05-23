// https://github.com/oxidize-rb/rb-sys/blob/main/crates/rb-sys-env/src/utils.rs#L2
macro_rules! rustc_cfg {
  ($enable:expr, $var:literal, $($cfg:tt)*) => {
      println!(concat!("cargo:rustc-check-cfg=cfg(", $var, ")"), $($cfg)*);
      if $enable {
          println!(concat!("cargo:rustc-cfg=", $var), $($cfg)*);
      }
  };
}

const COMPARABLE_MAJORS: [(i64, i64); 5] = [(1, 10), (2, 0), (2, 1), (2, 2), (2, 3)];
const COMPARABLE_PRODUCTS: [i64; 2] = [1, 2];

fn main() {
    // we should do this in the -sys crate but it doesn't work...?
    // this whole thing is really jank and does not work how i'd like

    // hardcode for docsrs
    #[cfg(not(docsrs))]
    let version_number = std::env::var("DEP_FMOD_VERSION_NUMBER").unwrap();
    #[cfg(docsrs)]
    let version_number = "131847"; // 0x20307
    let version_number: i64 = version_number.parse().unwrap();

    let minor = version_number & 0xFF;
    let major = (version_number >> 8) & 0xFF;
    let product = version_number >> 16;

    // https://github.com/oxidize-rb/rb-sys/blob/8548a8b6369494a4963eeeb6e08b918b0919fe08/crates/rb-sys-env/src/fmod_version.rs#L53
    rustc_cfg!(true, "fmod_{}", product);
    rustc_cfg!(true, "fmod_{}_{}", product, major);
    rustc_cfg!(true, "fmod_{}_{}_{}", product, major, minor);

    for v in &COMPARABLE_MAJORS {
        rustc_cfg!((product, major) < *v, r#"fmod_lt_{}_{}"#, v.0, v.1);
        rustc_cfg!((product, major) <= *v, r#"fmod_lte_{}_{}"#, v.0, v.1);
        rustc_cfg!((product, major) == *v, r#"fmod_{}_{}"#, v.0, v.1);
        rustc_cfg!((product, major) == *v, r#"fmod_eq_{}_{}"#, v.0, v.1);
        rustc_cfg!((product, major) >= *v, r#"fmod_gte_{}_{}"#, v.0, v.1);
        rustc_cfg!((product, major) > *v, r#"fmod_gt_{}_{}"#, v.0, v.1);
    }

    for v in &COMPARABLE_PRODUCTS {
        rustc_cfg!(major < *v, r#"fmod_lt_{}"#, v);
        rustc_cfg!(major <= *v, r#"fmod_lte_{}"#, v);
        rustc_cfg!(major == *v, r#"fmod_{}"#, v);
        rustc_cfg!(major == *v, r#"fmod_eq_{}"#, v);
        rustc_cfg!(major >= *v, r#"fmod_gte_{}"#, v);
        rustc_cfg!(major > *v, r#"fmod_gt_{}"#, v);
    }
}
