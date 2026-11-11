# Dev aid: saves a screenshot of one window, found by its title. For the post's images.
param([Parameter(Mandatory)][string]$Title, [Parameter(Mandatory)][string]$Out, [int]$Wait = 20, [switch]$Close)
Add-Type -AssemblyName System.Drawing
Add-Type -Name W -Namespace ShotWin -MemberDefinition @'
[DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
[DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
[DllImport("dwmapi.dll")] public static extern int DwmGetWindowAttribute(IntPtr h, int attr, out RECT r, int size);
[DllImport("user32.dll")] public static extern bool PostMessage(IntPtr h, uint msg, IntPtr w, IntPtr l);
public struct RECT { public int Left, Top, Right, Bottom; }
'@
[ShotWin.W]::SetProcessDPIAware() | Out-Null
$h = [IntPtr]::Zero
# By the processes' main windows: an exact FindWindow misses titles with direction marks.
for ($i = 0; $i -lt $Wait * 4 -and $h -eq [IntPtr]::Zero; $i++) {
    Start-Sleep -Milliseconds 250
    $p = Get-Process | Where-Object { $_.MainWindowTitle -like "*$Title*" } | Select-Object -First 1
    if ($p) { $h = $p.MainWindowHandle }
}
if ($h -eq [IntPtr]::Zero) { throw "no window '$Title'" }
[ShotWin.W]::SetForegroundWindow($h) | Out-Null
Start-Sleep -Milliseconds 1500
$r = New-Object ShotWin.W+RECT
[ShotWin.W]::DwmGetWindowAttribute($h, 9, [ref]$r, 16) | Out-Null   # 9 = the frame without the shadow
$bmp = New-Object Drawing.Bitmap ($r.Right - $r.Left), ($r.Bottom - $r.Top)
[Drawing.Graphics]::FromImage($bmp).CopyFromScreen($r.Left, $r.Top, 0, 0, $bmp.Size)
$bmp.Save($Out)
if ($Close) { [ShotWin.W]::PostMessage($h, 0x10, [IntPtr]::Zero, [IntPtr]::Zero) | Out-Null }   # WM_CLOSE
"$Out $($bmp.Width)x$($bmp.Height)"
