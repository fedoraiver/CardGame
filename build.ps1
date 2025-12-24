cargo build;
Remove-Item ./CardGame.exe -Force;
Move-Item ./target/debug/CardGame.exe ./CardGame.exe;
