echo "Building for 800 peeps";
cp /Users/jubitaneja/myrepo/mozilla/wasmtime/superopt/superopt_baseline_clang_800.rs ~/myrepo/mozilla/wasmtime/cranelift/codegen/src/superopt_baseline.rs
cd ~/myrepo/mozilla/wasmtime/cranelift/
cargo clean
cd ~/myrepo/mozilla/wasmtime/
cargo clean
cd ~/myrepo/mozilla/wasmtime/cranelift/
echo "\nBuild cranelift for 800 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-800.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-800.txt 2>&1
cd ~/myrepo/mozilla/wasmtime/
echo "\nBuild wasmtime for 800 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-800.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-800.txt 2>&1

echo "Building for 900 peeps";
cp /Users/jubitaneja/myrepo/mozilla/wasmtime/superopt/superopt_baseline_clang_900.rs ~/myrepo/mozilla/wasmtime/cranelift/codegen/src/superopt_baseline.rs
cd ~/myrepo/mozilla/wasmtime/cranelift/
cargo clean
cd ~/myrepo/mozilla/wasmtime/
cargo clean
cd ~/myrepo/mozilla/wasmtime/cranelift/
echo "\nBuild cranelift for 900 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-900.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-900.txt 2>&1
cd ~/myrepo/mozilla/wasmtime/
echo "\nBuild wasmtime for 900 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-900.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-900.txt 2>&1

echo "Building for 1000 peeps";
cp /Users/jubitaneja/myrepo/mozilla/wasmtime/superopt/superopt_baseline_clang_1000.rs ~/myrepo/mozilla/wasmtime/cranelift/codegen/src/superopt_baseline.rs
cd ~/myrepo/mozilla/wasmtime/cranelift/
cargo clean
cd ~/myrepo/mozilla/wasmtime/
cargo clean
cd ~/myrepo/mozilla/wasmtime/cranelift/
echo "\nBuild cranelift for 1000 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-1000.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-1000.txt 2>&1
cd ~/myrepo/mozilla/wasmtime/
echo "\nBuild wasmtime for 1000 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-1000.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-1000.txt 2>&1

echo "Building for 1100 peeps";
cp /Users/jubitaneja/myrepo/mozilla/wasmtime/superopt/superopt_baseline_clang_1100.rs ~/myrepo/mozilla/wasmtime/cranelift/codegen/src/superopt_baseline.rs
cd ~/myrepo/mozilla/wasmtime/cranelift/
cargo clean
cd ~/myrepo/mozilla/wasmtime/
cargo clean
cd ~/myrepo/mozilla/wasmtime/cranelift/
echo "\nBuild cranelift for 1100 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-1100.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-1100.txt 2>&1
cd ~/myrepo/mozilla/wasmtime/
echo "\nBuild wasmtime for 1100 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-1100.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-1100.txt 2>&1

echo "Building for 1200 peeps";
cp /Users/jubitaneja/myrepo/mozilla/wasmtime/superopt/superopt_baseline_clang_1200.rs ~/myrepo/mozilla/wasmtime/cranelift/codegen/src/superopt_baseline.rs
cd ~/myrepo/mozilla/wasmtime/cranelift/
cargo clean
cd ~/myrepo/mozilla/wasmtime/
cargo clean
cd ~/myrepo/mozilla/wasmtime/cranelift/
echo "\nBuild cranelift for 1200 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-1200.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-1200.txt 2>&1
cd ~/myrepo/mozilla/wasmtime/
echo "\nBuild wasmtime for 1200 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-1200.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-1200.txt 2>&1

echo "Building for 1300 peeps";
cp /Users/jubitaneja/myrepo/mozilla/wasmtime/superopt/superopt_baseline_clang_1300.rs ~/myrepo/mozilla/wasmtime/cranelift/codegen/src/superopt_baseline.rs
cd ~/myrepo/mozilla/wasmtime/cranelift/
cargo clean
cd ~/myrepo/mozilla/wasmtime/
cargo clean
cd ~/myrepo/mozilla/wasmtime/cranelift/
echo "\nBuild cranelift for 1300 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-1300.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-1300.txt 2>&1
cd ~/myrepo/mozilla/wasmtime/
echo "\nBuild wasmtime for 1300 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-1300.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-1300.txt 2>&1

echo "Building for 1413 peeps";
cp /Users/jubitaneja/myrepo/mozilla/wasmtime/superopt/superopt_baseline_clang_1413.rs ~/myrepo/mozilla/wasmtime/cranelift/codegen/src/superopt_baseline.rs
cd ~/myrepo/mozilla/wasmtime/cranelift/
cargo clean
cd ~/myrepo/mozilla/wasmtime/
cargo clean
cd ~/myrepo/mozilla/wasmtime/cranelift/
echo "\nBuild cranelift for 1413 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-1413.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-1413.txt 2>&1
cd ~/myrepo/mozilla/wasmtime/
echo "\nBuild wasmtime for 1413 peeps\n" >> ~/myrepo/mozilla/wasmtime/cranelift/log-1413.txt
cargo build --release >> ~/myrepo/mozilla/wasmtime/cranelift/log-1413.txt 2>&1

