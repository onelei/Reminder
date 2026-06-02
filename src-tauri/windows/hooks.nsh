; Uninstall: always remove local app data (config, etc.), skip during in-place updates.
!macro NSIS_HOOK_POSTUNINSTALL
  ${If} $UpdateMode <> 1
    SetShellVarContext current
    RmDir /r "$APPDATA\${BUNDLEID}"
    RmDir /r "$LOCALAPPDATA\${BUNDLEID}"
  ${EndIf}
!macroend
