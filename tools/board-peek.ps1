# Opens the widgets board (Win+W), waits, saves a screenshot of the screen, and closes the board (Esc).
# Dev aid only: to see the widgets as the board draws them.
param([string]$Out = "$env:TEMP\board-peek.png", [int]$Wait = 7)
Add-Type -AssemblyName System.Drawing, System.Windows.Forms
Add-Type -Name W -Namespace BoardPeek -MemberDefinition @'
[DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
[DllImport("user32.dll")] public static extern void keybd_event(byte vk, byte scan, uint flags, UIntPtr extra);
'@
[BoardPeek.W]::SetProcessDPIAware() | Out-Null
function Key([byte[]]$vks) { foreach ($k in $vks) { [BoardPeek.W]::keybd_event($k, 0, 0, [UIntPtr]::Zero) }; [array]::Reverse($vks); foreach ($k in $vks) { [BoardPeek.W]::keybd_event($k, 0, 2, [UIntPtr]::Zero) } }
Key 0x5B, 0x57          # Win+W
Start-Sleep $Wait
$r = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$bmp = New-Object Drawing.Bitmap $r.Width, $r.Height
[Drawing.Graphics]::FromImage($bmp).CopyFromScreen(0, 0, 0, 0, $bmp.Size)
$bmp.Save($Out)
Key 0x1B                # Esc
$Out
