; Eoliann Windows Tools v2 Inno Setup script
; Build first with: npm run tauri build

[Setup]
AppName=Eoliann Windows Tools
AppVersion=2.0.0
AppPublisher=eoliann
DefaultDirName={pf}\Eoliann Windows Tools
DefaultGroupName=Eoliann Windows Tools
Compression=lzma
SolidCompression=yes
OutputBaseFilename=Eoliann_Windows_Tools_v2_Installer
DisableProgramGroupPage=false
UninstallDisplayIcon={app}\eoliann-windows-tools.exe
PrivilegesRequired=admin

[Files]
; Adjust this path if Tauri changes the release executable location/name.
Source: "..\src-tauri\target\release\eoliann-windows-tools.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Eoliann Windows Tools"; Filename: "{app}\eoliann-windows-tools.exe"
Name: "{commondesktop}\Eoliann Windows Tools"; Filename: "{app}\eoliann-windows-tools.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop icon"; GroupDescription: "Additional icons:"

[Run]
Filename: "{app}\eoliann-windows-tools.exe"; Description: "Launch Eoliann Windows Tools"; Flags: nowait postinstall skipifsilent
