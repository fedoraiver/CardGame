cargo build;
Remove-Item ./demo2.exe -Force;
Move-Item ./target/debug/demo2.exe ./demo2.exe;
