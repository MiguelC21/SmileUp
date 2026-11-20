# Script de instalación para SmileUp (VST3 en Windows)
$vst3_dest = "$env:CommonProgramFiles\VST3"
$built_vst3 = "$PSScriptRoot\build\SmileUp_artefacts\Release\VST3\SmileUp.vst3"

if (!(Test-Path $vst3_dest)) {
    New-Item -ItemType Directory -Force -Path $vst3_dest | Out-Null
}

Write-Host "Instalando SmileUp en el sistema..." -ForegroundColor Cyan

if (Test-Path $built_vst3) {
    Copy-Item -Recurse -Force $built_vst3 "$vst3_dest\SmileUp.vst3"
    Copy-Item -Recurse -Force $built_vst3 "$vst3_dest\smile_up.vst3"
    if (Test-Path "$vst3_dest\smile_up.vst3\Contents\x86_64-win\SmileUp.vst3") {
        Copy-Item -Force "$vst3_dest\smile_up.vst3\Contents\x86_64-win\SmileUp.vst3" "$vst3_dest\smile_up.vst3\Contents\x86_64-win\smile_up.vst3"
    }
    Write-Host " [OK] SmileUp instalado en: $vst3_dest\SmileUp.vst3" -ForegroundColor Green
} else {
    Write-Host " [!] No se encontro el plugin compilado en $built_vst3. Compilalo primero con CMake." -ForegroundColor Yellow
}

Write-Host "`nInstalacion completada! Ya puedes abrir tu DAW y disfrutar de SmileUp." -ForegroundColor Magenta
