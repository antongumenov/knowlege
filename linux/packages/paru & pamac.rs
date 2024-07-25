// paru
sudo pacman -S base-devel git;
git clone https://aur.archlinux.org/paru-bin.git;
cd paru-bin;
makepkg -sricCf;
cd ..;
rm -rf paru-bin;
paru -S pamac


