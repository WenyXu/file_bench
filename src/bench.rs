extern crate test;

#[cfg(test)]
mod tests {
    use super::test::Bencher;
    use std::collections::BTreeMap;
    use std::fs::{create_dir, remove_dir, File};
    use std::io::Write;
    use std::sync::atomic::AtomicI32;

    const DATA_SCALE: usize = 1024;
    const LOG_FILE_SCALE: i32 = 1;
    const TEST_WAL_DIR: &str = "./__test";

    #[bench]
    fn bench_single_file(b: &mut Bencher) {
        let _ = remove_dir(TEST_WAL_DIR);
        let _ = create_dir(TEST_WAL_DIR);
        // Creates a single file.
        let path = "./__test/single.wal";
        let mut single_file = File::create(path).unwrap();

        // 1024 Bytes * scale
        let buf = [0u8; 1024 * DATA_SCALE];

        b.iter(|| single_file.write_all(&buf[..]));
    }

    #[bench]
    fn bench_multiple_files(b: &mut Bencher) {
        let _ = remove_dir(TEST_WAL_DIR);
        let _ = create_dir(TEST_WAL_DIR);

        // Creates multiple files.
        let mut map = BTreeMap::new();
        let total = 1_000 * LOG_FILE_SCALE;
        for i in 0..total {
            let path = format!("./__test/mutli-{}.wal", i);
            let file = File::create(path).unwrap();
            map.insert(i, file);
        }

        // 1024 Bytes * scale
        let buf = [0u8; 1024 * DATA_SCALE];

        // A naive load balance
        let cur = AtomicI32::new(0);

        b.iter(|| {
            let old = cur.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let target = map.get_mut(&(old % total)).unwrap();
            target.write_all(&buf[..]).unwrap();
        });
    }
}
