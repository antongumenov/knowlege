//~ ставлю cups
// paru -S cups
// sudo systemctl enable --now cups
// usermod -aG lp ang

//~ иду на вики и ставлю драйвер (splix)

//~ добавляю принтер
lpadmin -p SCX-4300 -E -v "usb://Samsung/SCX-4300%20Series?serial=9N66BFFQ501399Z.&interface=1" -m lsb/usr/suld/Samsung_SCX-4300_Series.ppd.gz

//  SCX-4300 - это имя, сам пишу какое хочу
// "usb://Samsung/SCX-4300%20Series?serial=9N66BFFQ501399Z.&interface=1" - uri устройства, добывается командой lpinfo -v
//  lsb/usr/suld/Samsung_SCX-4300_Series.ppd.gz - model устройствa, добывается командой lpinfo -p

