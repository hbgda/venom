cd venom_core
cargo build --release
Copy-Item "./target/release/venom_core.dll" -Destination "C:\Program Files (x86)\Steam\steamapps\common\Marvel's Spider-Man Miles Morales\mods\scripts\core\" -Force

# cd ../venom_keybinds
# cargo build --release
# Copy-Item "./target/release/venom_keybinds.dll" -Destination "C:\Program Files (x86)\Steam\steamapps\common\Marvel's Spider-Man Miles Morales\mods\scripts\core" -Force

cd ../venom_menu
cargo build --release
Copy-Item "./target/release/venom_menu.dll" -Destination "C:\Program Files (x86)\Steam\steamapps\common\Marvel's Spider-Man Miles Morales\mods\scripts\core" -Force

cd ../test_script
cargo build --release
Copy-Item "./target/release/test_script.dll" -Destination "C:\Program Files (x86)\Steam\steamapps\common\Marvel's Spider-Man Miles Morales\mods\scripts\" -Force

cd ..