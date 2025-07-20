#[macro_export]
macro_rules! require_files {
    ( $mode:ident, $( $file:expr ),+ $(,)? ) => {
        {
            let mode_str = stringify!($mode);

            $(
                let path = std::path::Path::new($file);

                match mode_str {
                    "exists" => {
                        if !path.exists() {
                            anyhow::bail!("❌ Required file missing: {}", $file);
                        }
                    }
                    "missing" => {
                        if path.exists() {
                            anyhow::bail!("❌ File {} must not exist ", $file);
                        }
                    }
                    _ => {
                        panic!("Invalid mode in require_files!: use exists or missing");
                    }
                }
            )+
        }
    };
}
