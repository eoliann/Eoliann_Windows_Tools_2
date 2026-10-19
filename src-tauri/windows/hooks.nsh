; Installer hooks for Eoliann Windows Tools 2.
;
; Up to v1.1.0 this app was installed as "Eoliann Windows Tools" - the same name as
; Eoliann Windows Tools v1 - so both installers wrote the same desktop / Start menu
; shortcut and each one replaced the other. From v1.1.1 the product is named
; "Eoliann Windows Tools 2"; these hooks migrate the old per-user install.

!define EWT_OLD_UNINSTKEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\Eoliann Windows Tools"
; Eoliann Windows Tools v1 (Inno Setup, per-machine). The doubled brace is part of its AppId.
!define EWT_V1_UNINSTKEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\{7E6F0C4D-2C2A-4B16-9B8D-EOLIANN135}}_is1"

Var EwtRemovedOldInstall

!macro NSIS_HOOK_PREINSTALL
  Push $R0
  Push $R1
  StrCpy $EwtRemovedOldInstall 0

  ; The old install of this app is a Tauri/NSIS per-user install with an uninstall.exe.
  ; Eoliann Windows Tools v1 uses a different uninstall key (Inno Setup) and is not touched.
  ReadRegStr $R0 HKCU "${EWT_OLD_UNINSTKEY}" "InstallLocation"
  ${If} $R0 != ""
    StrCpy $R1 $R0 1
    ${If} $R1 == '"'
      StrCpy $R0 $R0 "" 1
      StrCpy $R0 $R0 -1
    ${EndIf}
    ${If} $R0 != $INSTDIR
    ${AndIf} ${FileExists} "$R0\uninstall.exe"
      DetailPrint "Removing the previous install of Eoliann Windows Tools 2 from $R0"
      ; _?= keeps the uninstaller in place so ExecWait waits for it to finish.
      ExecWait '"$R0\uninstall.exe" /S _?=$R0'
      Delete "$R0\uninstall.exe"
      RMDir "$R0"
      DeleteRegKey HKCU "${EWT_OLD_UNINSTKEY}"
      StrCpy $EwtRemovedOldInstall 1
    ${EndIf}
  ${EndIf}

  Pop $R1
  Pop $R0
!macroend

!macro NSIS_HOOK_POSTINSTALL
  ; The old install overwrote v1's desktop shortcut; give it back if v1 is installed.
  ${If} $EwtRemovedOldInstall == 1
    Push $R0
    SetRegView 64
    ReadRegStr $R0 HKLM "${EWT_V1_UNINSTKEY}" "InstallLocation"
    SetRegView lastused
    ${If} $R0 != ""
    ${AndIf} ${FileExists} "$R0eoliann_windows_tools_w11.exe"
    ${AndIfNot} ${FileExists} "$DESKTOP\Eoliann Windows Tools.lnk"
      CreateShortcut "$DESKTOP\Eoliann Windows Tools.lnk" "$R0eoliann_windows_tools_w11.exe" "" "$R0icon.ico" 0
    ${EndIf}
    Pop $R0
  ${EndIf}
!macroend
