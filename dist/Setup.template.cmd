<# :
@echo off
rem HaFeed for the Windows widgets board - single-file installer.
rem No argument: the GUI (runs as the user). With an argument: one elevated policy step, called by the GUI.
rem /quiet: the whole install with no window (for technicians). Exit code 0 = done, 3010 = done, restart needed,
rem 1 = failed (details in %ProgramData%\HagizraFeed\setup.log). Windows asks for admin unless already elevated.
set "HZ_SELF=%~f0"
set "HZ_MODE=%~1"
set "HZ_PS1=%LOCALAPPDATA%\HagizraFeed\setup.ps1"
rem The script part runs as a file (-File), not as text (antivirus programs block "iex" of text read from a file).
rem This file is a valid .ps1 too; copied with a BOM so Windows PowerShell reads the Hebrew as UTF-8.
powershell.exe -NoProfile -Command "[void][IO.Directory]::CreateDirectory([IO.Path]::GetDirectoryName($env:HZ_PS1)); [IO.File]::WriteAllText($env:HZ_PS1, [IO.File]::ReadAllText($env:HZ_SELF, [Text.Encoding]::UTF8), [Text.UTF8Encoding]::new($true))"
if /i "%~1"=="/quiet" goto quiet
if "%~1"=="" goto gui
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%HZ_PS1%"
exit /b %errorlevel%
:quiet
set "HZ_MODE=quiet"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%HZ_PS1%"
exit /b %errorlevel%
:gui
start "" conhost.exe --headless powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%HZ_PS1%"
exit /b
#>
# ====================================================================================
#  הפיד בלוח הווידג'טים - מתקין בקובץ אחד.
#  1. פותח פידים של צד שלישי במחשב (שינוי בקובץ מדיניות האזור, עם גיבוי), ומוסיף את תעודת
#     החתימה של החבילה ל-TrustedPeople של המחשב. דורש מנהל, בבקשה אחת.
#  2. מתקין את Windows App Runtime 1.6 אם חסר.
#  3. מתקין את חבילת הפיד (MSIX חתום בתעודה עצמית). חבילה לא חתומה לא יכולה להריץ exe.
# ====================================================================================
$ErrorActionPreference = 'Stop'
# The admin step gets its mode, result file and the installer's path as arguments: an elevated process does
# not inherit the environment.
if ($args.Count -ge 3) { $env:HZ_MODE = $args[0]; $env:HZ_OUT = $args[1]; $env:HZ_SELF = $args[2] }
$Self    = $env:HZ_SELF
$Version = '{{VERSION}}'
$PkgName = 'HagizraFeed'
$CertThumb = '{{CERT}}'
$Policy  = if ($env:HZ_POLICY) { $env:HZ_POLICY } else { "$env:windir\System32\IntegratedServicesRegionPolicySet.json" }
$LogFile = "$env:ProgramData\HagizraFeed\setup.log"

function Write-Log($m) {
    try { New-Item -ItemType Directory -Force (Split-Path $LogFile) | Out-Null
          Add-Content -Path $LogFile -Value "$(Get-Date -Format 'yyyy-MM-dd HH:mm:ss') [$env:USERNAME] $m" -Encoding UTF8 } catch { }
}

function Get-Data($tag) {
    $m = [regex]::Match([IO.File]::ReadAllText($Self), "(?m)^$tag`:([A-Za-z0-9+/=]+)\s*$")
    if (-not $m.Success) { throw "חסר מידע ($tag) בקובץ ההתקנה" }
    [Convert]::FromBase64String($m.Groups[1].Value)
}
function Test-Trusted { Test-Path "Cert:\LocalMachine\TrustedPeople\$CertThumb" }

# ---------- מדיניות האזור ----------
# הכללים שמיקרוסופט פותחת רק באזור הכלכלי האירופי.
$Rules = '16d2b50e-fa7c-4bb1-ab17-01d766530b3b',   # Third party feed is shown in Widgets
         '92ca7dcf-f422-4a4b-892e-7883a3ca8d70',   # First party Widget feed can be disabled
         '3d0c862c-bfd8-4350-af2f-4c42884cc3a8'    # Use default web protocol for Widget links (קישורים בדפדפן ברירת המחדל, לא ב-Edge)

function Get-RegionCode {
    try {
        Add-Type -Namespace HZ -Name Geo -MemberDefinition '[DllImport("kernel32.dll", CharSet=CharSet.Unicode)] public static extern int GetGeoInfoW(int id, int type, System.Text.StringBuilder data, int len, ushort lang);' -ErrorAction SilentlyContinue
        $id = (Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows\CurrentVersion\Control Panel\DeviceRegion').DeviceRegion
        $sb = [Text.StringBuilder]::new(8)
        if ([HZ.Geo]::GetGeoInfoW([int]$id, 4, $sb, 8, 0) -gt 0 -and $sb.ToString() -match '^[A-Z]{2}$') { return $sb.ToString() }   # 4 = GEO_ISO2
    } catch { }
    'IL'
}

# מוסיף את קוד האזור לרשימת ה-enabled של כל כלל. לא נוגע בשום דבר אחר בקובץ.
function Get-PatchedPolicy([string]$Text, [string]$Cc) {
    foreach ($g in $Rules) {
        $i = $Text.IndexOf("{$g}", [StringComparison]::OrdinalIgnoreCase)
        if ($i -lt 0) { throw "הכלל $g לא נמצא בקובץ המדיניות" }
        $j = $Text.IndexOf('"enabled"', $i)
        $n = $Text.IndexOf('"guid"', $i + 1)
        if ($j -lt 0 -or ($n -ge 0 -and $n -lt $j)) { throw "מבנה לא מוכר בכלל $g" }
        $k = $Text.IndexOf('[', $j); $e = $Text.IndexOf(']', $k)
        if ($Text.Substring($k, $e - $k) -notmatch "`"$Cc`"") { $Text = $Text.Insert($k + 1, "`"$Cc`", ") }
    }
    $null = $Text | ConvertFrom-Json   # לעולם לא לכתוב קובץ מערכת שבור
    $Text
}

function Read-Policy { $b = [IO.File]::ReadAllBytes($Policy); @{ Bom = ($b.Length -ge 3 -and $b[0] -eq 0xEF -and $b[1] -eq 0xBB -and $b[2] -eq 0xBF); Text = [IO.File]::ReadAllText($Policy) } }
function Test-PolicyOpen { try { $p = Read-Policy; (Get-PatchedPolicy $p.Text (Get-RegionCode)) -eq $p.Text } catch { $false } }

function Unlock-File($f) {
    if (-not (Test-Path $f)) { return }
    $ErrorActionPreference = 'Continue'   # הפלט של takeown ו-icacls הוא לא שגיאה
    takeown.exe /f $f /a 2>&1 | Out-Null
    icacls.exe $f /grant '*S-1-5-32-544:F' 2>&1 | Out-Null
}
# ponytail: מחזירים רק את הבעלות ל-TrustedInstaller. הרשאת המנהלים שנוספה נשארת על הקובץ.
function Lock-File($f) { $ErrorActionPreference = 'Continue'; if (Test-Path $f) { icacls.exe $f /setowner 'NT SERVICE\TrustedInstaller' 2>&1 | Out-Null } }

# קודי יציאה: 0 שונה, 1 כבר פתוח, 2 שוחזר, 3 אין מה לשחזר, 10 כשל.
function Invoke-PolicyOpen {
    $cc = Get-RegionCode
    $p = Read-Policy
    $new = Get-PatchedPolicy $p.Text $cc
    if ($new -eq $p.Text) { Write-Log "policy already open for $cc"; return 1 }
    $orig = "$Policy.orig"
    Unlock-File $Policy; Unlock-File $orig
    # תוקן כבר בעבר (גרסה קודמת של המתקין): הגיבוי הקיים הוא המקור של אותו תיקון, ונשאר.
    $keep = (Test-Path $orig) -and $(try { (Get-PatchedPolicy ([IO.File]::ReadAllText($orig)) $cc) -eq $new } catch { $false })
    if ($keep) {
        $old = "$Policy.hagizra-old"
        if (Test-Path $old) { Unlock-File $old; Remove-Item $old -Force -ErrorAction SilentlyContinue }
        Rename-Item $Policy (Split-Path $old -Leaf)
    } else {
        if (Test-Path $orig) { Remove-Item $orig -Force }      # גיבוי ישן מלפני עדכון של Windows
        Rename-Item $Policy (Split-Path $orig -Leaf)           # הקובץ בשימוש: שינוי שם עובד, כתיבה במקום לא
    }
    [IO.File]::WriteAllText($Policy, $new, [Text.UTF8Encoding]::new($p.Bom))
    Lock-File $Policy; Lock-File $orig
    if ($keep) { Remove-Item "$Policy.hagizra-old" -Force -ErrorAction SilentlyContinue }   # בשימוש עד הפעלה מחדש: נשאר, לא מזיק
    Write-Log "policy opened for $cc (backup: $orig, kept: $keep)"
    0
}

function Invoke-PolicyRestore {
    $orig = "$Policy.orig"
    if (-not (Test-Path $orig)) { Write-Log 'policy restore: no backup'; return 3 }
    Unlock-File $Policy; Unlock-File $orig
    if (-not (Test-PolicyOpen)) {
        # Windows כבר החליף את הקובץ בגרסה חדשה. הגיבוי ישן יותר ממנו.
        Remove-Item $orig -Force; Write-Log 'policy restore: file already stock, dropped old backup'; return 3
    }
    $old = "$Policy.hagizra-old"
    if (Test-Path $old) { Unlock-File $old; Remove-Item $old -Force -ErrorAction SilentlyContinue }
    Rename-Item $Policy (Split-Path $old -Leaf)
    Rename-Item $orig (Split-Path $Policy -Leaf)
    Lock-File $Policy
    Remove-Item $old -Force -ErrorAction SilentlyContinue      # עדיין בשימוש עד הפעלה מחדש: נשאר, לא מזיק
    Write-Log 'policy restored from backup'
    2
}

# ---------- תעודת החתימה ----------
function Install-Cert {
    if (Test-Trusted) { return }
    $cer = Join-Path $env:TEMP 'hagizra-feed.cer'
    [IO.File]::WriteAllBytes($cer, (Get-Data 'CERT'))
    certutil.exe -f -addstore TrustedPeople $cer | Out-Null; $failed = $LASTEXITCODE
    Remove-Item $cer -Force -ErrorAction SilentlyContinue
    if ($failed -or -not (Test-Trusted)) { throw "certutil addstore failed ($failed)" }
    Write-Log "certificate $CertThumb trusted"
}
function Remove-Cert {
    if (-not (Test-Trusted)) { return }
    certutil.exe -delstore TrustedPeople $CertThumb | Out-Null
    Write-Log "certificate $CertThumb removed"
}

# ---------- מצב מנהל: צעד אחד ויציאה ----------
if ($env:HZ_MODE -and $env:HZ_MODE -ne 'quiet') {
    try {
        $code = switch ($env:HZ_MODE) {
            'setup'  { Install-Cert; Invoke-PolicyOpen }
            'remove' { Remove-Cert; Invoke-PolicyRestore }
            default  { 10 }
        }
    } catch { Write-Log "$($env:HZ_MODE) failed: $_"; $code = 10 }
    if ($env:HZ_OUT) {
        Get-ChildItem (Split-Path $env:HZ_OUT) -Filter 'step-*.txt' -ErrorAction SilentlyContinue | Remove-Item -ErrorAction SilentlyContinue
        try { Set-Content $env:HZ_OUT $code } catch {}
    }
    exit $code
}

# ====================================================================================
#  ממשק
# ====================================================================================
function Get-Runtime { Get-AppxPackage Microsoft.WindowsAppRuntime.1.6 -ErrorAction SilentlyContinue | Where-Object { $_.Architecture -eq 'X64' -and [version]$_.Version -ge [version]'6000.519.329.0' } | Select-Object -First 1 }

Add-Type -AssemblyName PresentationFramework, PresentationCore, WindowsBase
# The window runs with no console: anything that stops it from opening goes to the log and to a message.
trap {
    Write-Log "window failed: $_ (line $($_.InvocationInfo.ScriptLineNumber))"
    if ($env:HZ_MODE -ne 'quiet') { [void][Windows.MessageBox]::Show("המתקין נתקל בשגיאה ולא נפתח.`n`n$_`n`nהפרטים נשמרו ב-$LogFile", 'הפיד') }
    exit 1
}

$build = [Environment]::OSVersion.Version.Build
$light = (Get-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize' -Name AppsUseLightTheme -ErrorAction SilentlyContinue).AppsUseLightTheme -ne 0
$mica  = $build -ge 22621
# צבעי הלוגו של הפיד: כחול #00609C ואדום #FF4D4F.
$T = if ($light) { @{
    FG='#1A1A1A'; FG2='#616161'; WINBG='#F3F3F3'; CARD='#C8FFFFFF'; CARDBORDER='#14000000'; FOOTER='#08000000'
    CTRL='#C8FFFFFF'; CTRLBORDER='#1F000000'; ACCENT='#00609C'; ACCENTSOFT='#1A00609C'; ONACCENT='#FFFFFF'
    RED='#E5484D'; GOOD='#0F7B0F'; BAD='#C42B1C'; TRACK='#1F000000'; UACBG='#FFFFFF'; UACBTN='#0067C0'; UACON='#FFFFFF' }
} else { @{
    FG='#FFFFFF'; FG2='#C5C5C5'; WINBG='#202020'; CARD='#0FFFFFFF'; CARDBORDER='#1AFFFFFF'; FOOTER='#1F000000'
    CTRL='#12FFFFFF'; CTRLBORDER='#1AFFFFFF'; ACCENT='#4FA8E8'; ACCENTSOFT='#264FA8E8'; ONACCENT='#00121F'
    RED='#FF5A50'; GOOD='#6CCB5F'; BAD='#FF99A4'; TRACK='#26FFFFFF'; UACBG='#2B2B2B'; UACBTN='#4CC2FF'; UACON='#000000' }
}
if ($mica) { $T.WINBG = 'Transparent' }

$xaml = @'
<Window xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
        xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
        Title="הפיד - התקנה" Width="640" Height="620" ResizeMode="CanMinimize"
        WindowStartupLocation="CenterScreen" FlowDirection="RightToLeft" UseLayoutRounding="True"
        FontFamily="Segoe UI" FontSize="14" Foreground="{{FG}}" Background="{{WINBG}}">
  <WindowChrome.WindowChrome>
    <WindowChrome CaptionHeight="40" GlassFrameThickness="{{GLASS}}" ResizeBorderThickness="0" UseAeroCaptionButtons="True"/>
  </WindowChrome.WindowChrome>
  <Window.Resources>
    <Style x:Key="Btn" TargetType="Button">
      <Setter Property="Height" Value="36"/><Setter Property="MinWidth" Value="112"/><Setter Property="Padding" Value="18,0"/>
      <Setter Property="Cursor" Value="Hand"/><Setter Property="Foreground" Value="{{FG}}"/>
      <Setter Property="Background" Value="{{CTRL}}"/><Setter Property="BorderBrush" Value="{{CTRLBORDER}}"/>
      <Setter Property="Template"><Setter.Value>
        <ControlTemplate TargetType="Button">
          <Border x:Name="b" CornerRadius="6" Background="{TemplateBinding Background}" BorderBrush="{TemplateBinding BorderBrush}"
                  BorderThickness="1" Padding="{TemplateBinding Padding}">
            <ContentPresenter HorizontalAlignment="Center" VerticalAlignment="Center"/>
          </Border>
          <ControlTemplate.Triggers>
            <Trigger Property="IsMouseOver" Value="True"><Setter TargetName="b" Property="Opacity" Value="0.88"/></Trigger>
            <Trigger Property="IsPressed" Value="True"><Setter TargetName="b" Property="Opacity" Value="0.72"/></Trigger>
            <Trigger Property="IsEnabled" Value="False"><Setter TargetName="b" Property="Opacity" Value="0.4"/></Trigger>
          </ControlTemplate.Triggers>
        </ControlTemplate>
      </Setter.Value></Setter>
    </Style>
    <Style x:Key="Accent" TargetType="Button" BasedOn="{StaticResource Btn}">
      <Setter Property="Background" Value="{{ACCENT}}"/><Setter Property="BorderBrush" Value="{{ACCENT}}"/>
      <Setter Property="Foreground" Value="{{ONACCENT}}"/><Setter Property="FontWeight" Value="SemiBold"/>
    </Style>
    <Style x:Key="Glyph" TargetType="TextBlock">
      <Setter Property="FontFamily" Value="Segoe Fluent Icons, Segoe MDL2 Assets"/><Setter Property="FlowDirection" Value="LeftToRight"/>
      <Setter Property="VerticalAlignment" Value="Center"/>
    </Style>
    <Style x:Key="Card" TargetType="Border">
      <Setter Property="CornerRadius" Value="8"/><Setter Property="Background" Value="{{CARD}}"/>
      <Setter Property="BorderBrush" Value="{{CARDBORDER}}"/><Setter Property="BorderThickness" Value="1"/>
    </Style>
  </Window.Resources>

  <Grid>
    <Grid.RowDefinitions><RowDefinition Height="40"/><RowDefinition Height="Auto"/><RowDefinition/><RowDefinition Height="Auto"/></Grid.RowDefinitions>

    <TextBlock Text="התקנה" FontSize="12" Foreground="{{FG2}}" Margin="18,0" VerticalAlignment="Center"/>

    <!-- כותרת עם הלוגו -->
    <Grid Grid.Row="1" Margin="28,4,28,18">
      <Grid.ColumnDefinitions><ColumnDefinition Width="Auto"/><ColumnDefinition/></Grid.ColumnDefinitions>
      <Grid Width="68" Height="68">
        <Ellipse Fill="{{CARD}}" Stroke="{{CARDBORDER}}" StrokeThickness="1"/>
        <Image x:Name="Logo" Width="60" Height="60" RenderOptions.BitmapScalingMode="HighQuality">
          <Image.Clip><EllipseGeometry Center="30,30" RadiusX="30" RadiusY="30"/></Image.Clip>
        </Image>
      </Grid>
      <StackPanel Grid.Column="1" Margin="16,0,0,0" VerticalAlignment="Center">
        <StackPanel Orientation="Horizontal">
          <TextBlock Text="הפיד" FontFamily="Segoe UI" FontSize="28" FontWeight="Bold"/>
          <Border CornerRadius="10" Background="{{ACCENTSOFT}}" Padding="9,2" Margin="12,4,0,0" VerticalAlignment="Center">
            <TextBlock x:Name="VerChip" FontSize="12" Foreground="{{ACCENT}}" FlowDirection="LeftToRight"/>
          </Border>
        </StackPanel>
        <TextBlock Text="ערוצי חדשות ועדכונים, בפיד אחד בלוח הווידג'טים של Windows" Foreground="{{FG2}}" Margin="0,2,0,0"/>
      </StackPanel>
    </Grid>

    <Grid Grid.Row="2" Margin="28,0,28,20">

      <!-- ===== פתיחה ===== -->
      <StackPanel x:Name="PageHome">
        <WrapPanel x:Name="Chips" Margin="0,0,0,14"/>
        <Border Style="{StaticResource Card}" Padding="4">
          <StackPanel x:Name="Plan"/>
        </Border>
        <TextBlock x:Name="HomeNote" TextWrapping="Wrap" Foreground="{{FG2}}" FontSize="12.5" Margin="4,14,4,0" LineHeight="19"/>
      </StackPanel>

      <!-- ===== לפני בקשת המנהל: הסבר ותמונה של החלון של Windows ===== -->
      <StackPanel x:Name="PageUac" Visibility="Collapsed">
        <TextBlock x:Name="UacTitle" Text="אישור מנהל, פעם אחת" FontFamily="Segoe UI" FontSize="20" FontWeight="SemiBold" Margin="2,0,0,8"/>
        <TextBlock x:Name="UacText" TextWrapping="Wrap" Foreground="{{FG2}}" LineHeight="21" Margin="2,0,2,18"/>
        <Grid HorizontalAlignment="Center">
          <Border Width="380" CornerRadius="8" Background="{{UACBG}}" BorderBrush="{{CARDBORDER}}" BorderThickness="1" Margin="0,0,0,8">
            <Border.Effect><DropShadowEffect BlurRadius="22" ShadowDepth="4" Opacity="0.28"/></Border.Effect>
            <StackPanel>
              <TextBlock Text="בקרת חשבון משתמש" FontSize="12" Foreground="{{FG2}}" Margin="18,12,18,0"/>
              <TextBlock Text="האם ברצונך לאפשר לאפליקציה זו לבצע שינויים במכשיר שלך?" TextWrapping="Wrap" FontSize="17" FontWeight="SemiBold" Margin="18,8,18,16"/>
              <StackPanel Orientation="Horizontal" Margin="18,0,18,0">
                <Border Width="32" Height="32" CornerRadius="4" Background="#1E1E1E">
                  <TextBlock Style="{StaticResource Glyph}" Text="&#xE756;" FontSize="16" Foreground="White" HorizontalAlignment="Center"/>
                </Border>
                <TextBlock Text="Console Window Host" FontSize="15" Margin="12,0,0,0" VerticalAlignment="Center"/>
              </StackPanel>
              <TextBlock Text="מפרסם מאומת: Microsoft Windows" FontSize="12.5" Foreground="{{FG2}}" Margin="18,10,18,18"/>
              <Grid Margin="18,0,18,18">
                <Grid.ColumnDefinitions><ColumnDefinition/><ColumnDefinition Width="8"/><ColumnDefinition/></Grid.ColumnDefinitions>
                <Border Grid.Column="0" Height="32" CornerRadius="4" Background="{{UACBTN}}">
                  <TextBlock Text="כן" Foreground="{{UACON}}" HorizontalAlignment="Center" VerticalAlignment="Center"/>
                </Border>
                <Border Grid.Column="2" Height="32" CornerRadius="4" Background="{{CTRL}}" BorderBrush="{{CTRLBORDER}}" BorderThickness="1">
                  <TextBlock Text="לא" HorizontalAlignment="Center" VerticalAlignment="Center"/>
                </Border>
                <Border x:Name="UacRing" Grid.Column="0" Margin="-6" CornerRadius="9" BorderBrush="{{RED}}" BorderThickness="2.5"/>
              </Grid>
            </StackPanel>
          </Border>
        </Grid>
        <TextBlock Text="דוגמה. החלון האמיתי של Windows יופיע אחרי &quot;המשך&quot;." HorizontalAlignment="Center" FontSize="12" Foreground="{{FG2}}" Margin="0,4,0,0"/>
      </StackPanel>

      <!-- ===== עבודה ===== -->
      <StackPanel x:Name="PageWork" Visibility="Collapsed">
        <TextBlock x:Name="WorkTitle" Text="מתקין" FontFamily="Segoe UI" FontSize="20" FontWeight="SemiBold" Margin="2,0,0,12"/>
        <ProgressBar x:Name="Prog" Height="4" Minimum="0" Maximum="100" Foreground="{{ACCENT}}" Background="{{TRACK}}" Margin="2,0,2,18">
          <ProgressBar.Template><ControlTemplate TargetType="ProgressBar"><Grid>
            <Border x:Name="PART_Track" CornerRadius="2" Background="{TemplateBinding Background}"/>
            <Border x:Name="PART_Indicator" CornerRadius="2" Background="{TemplateBinding Foreground}" HorizontalAlignment="Left"/>
          </Grid></ControlTemplate></ProgressBar.Template>
        </ProgressBar>
        <Border Style="{StaticResource Card}" Padding="4">
          <StackPanel x:Name="Steps"/>
        </Border>
        <TextBlock x:Name="WorkError" TextWrapping="Wrap" Foreground="{{BAD}}" Margin="4,14,4,0" Visibility="Collapsed"/>
      </StackPanel>

      <!-- ===== סיום ===== -->
      <StackPanel x:Name="PageDone" Visibility="Collapsed">
        <StackPanel Orientation="Horizontal" Margin="2,0,0,14">
          <Grid Width="34" Height="34">
            <Ellipse x:Name="DoneDisc" Fill="{{GOOD}}"/>
            <TextBlock x:Name="DoneGlyph" Style="{StaticResource Glyph}" Text="&#xE73E;" FontSize="16" Foreground="White" HorizontalAlignment="Center"/>
          </Grid>
          <TextBlock x:Name="DoneTitle" FontFamily="Segoe UI" FontSize="20" FontWeight="SemiBold" Margin="12,0,0,0" VerticalAlignment="Center"/>
        </StackPanel>
        <TextBlock x:Name="DoneText" TextWrapping="Wrap" Foreground="{{FG2}}" Margin="2,0,2,14" LineHeight="21"/>
        <StackPanel x:Name="Guide"/>
      </StackPanel>
    </Grid>

    <!-- פוטר -->
    <Border Grid.Row="3" Background="{{FOOTER}}" BorderBrush="{{CARDBORDER}}" BorderThickness="0,1,0,0" Padding="28,16">
      <Grid>
        <StackPanel Orientation="Horizontal" HorizontalAlignment="Right">
          <Button x:Name="BtnPrimary" Style="{StaticResource Accent}"/>
          <Button x:Name="BtnSecondary" Style="{StaticResource Btn}" Margin="8,0,0,0" Visibility="Collapsed"/>
        </StackPanel>
        <Button x:Name="BtnClose" Style="{StaticResource Btn}" Content="סגירה" HorizontalAlignment="Left"/>
      </Grid>
    </Border>
  </Grid>
</Window>
'@
foreach ($k in $T.Keys) { $xaml = $xaml.Replace("{{$k}}", $T[$k]) }
$xaml = $xaml.Replace('{{GLASS}}', $(if ($mica) { '-1' } else { '0' }))
$win = [Windows.Markup.XamlReader]::Parse($xaml)
$ui = @{}
foreach ($n in 'Logo','VerChip','Chips','Plan','HomeNote','PageHome','PageUac','UacTitle','UacText','UacRing','PageWork','WorkTitle','WorkError','Prog','Steps','PageDone','DoneDisc','DoneGlyph','DoneTitle','DoneText','Guide','BtnPrimary','BtnSecondary','BtnClose') { $ui[$n] = $win.FindName($n) }
$ErrorActionPreference = 'Continue'   # שגיאה ב-handler לא מפילה את החלון
function Brush($c) { [Windows.Media.BrushConverter]::new().ConvertFromString($c) }

if ($mica) {
    Add-Type -Namespace HZ -Name Dwm -MemberDefinition '[DllImport("dwmapi.dll")] public static extern int DwmSetWindowAttribute(IntPtr h, int a, ref int v, int s);' -ErrorAction SilentlyContinue
    $win.Add_SourceInitialized({
        $h = [Windows.Interop.WindowInteropHelper]::new($win).Handle
        [Windows.Interop.HwndSource]::FromHwnd($h).CompositionTarget.BackgroundColor = [Windows.Media.Colors]::Transparent
        $dark = [int](-not $light); $bd = 2
        [HZ.Dwm]::DwmSetWindowAttribute($h, 20, [ref]$dark, 4) | Out-Null   # כותרת כהה
        [HZ.Dwm]::DwmSetWindowAttribute($h, 38, [ref]$bd, 4) | Out-Null     # Mica
    })
}

try {
    $img = [Windows.Media.Imaging.BitmapImage]::new()
    $img.BeginInit(); $img.StreamSource = [IO.MemoryStream]::new((Get-Data 'LOGO')); $img.CacheOption = 'OnLoad'; $img.EndInit()
    $ui.Logo.Source = $img
} catch { }
$ui.VerChip.Text = "v$Version"

# ---------- רכיבים ----------
function New-Glyph($code, $size, $color) {
    $g = [Windows.Controls.TextBlock]@{ Text = [string][char]$code; FontSize = $size }
    $g.Style = $win.FindResource('Glyph'); $g.Foreground = Brush $color; $g
}
function New-Chip($ok, $text) {
    $b = [Windows.Controls.Border]@{ CornerRadius = 14; Padding = '10,5,12,5'; Margin = '0,0,8,8'; BorderThickness = 1 }
    $b.Background = Brush $T.CARD; $b.BorderBrush = Brush $T.CARDBORDER
    $sp = [Windows.Controls.StackPanel]@{ Orientation = 'Horizontal' }
    $color = if ($ok -eq $true) { $T.GOOD } elseif ($ok -eq $false) { $T.BAD } else { $T.ACCENT }
    $glyph = if ($ok -eq $true) { 0xE73E } elseif ($ok -eq $false) { 0xE711 } else { 0xE946 }
    [void]$sp.Children.Add((New-Glyph $glyph 12 $color))
    [void]$sp.Children.Add([Windows.Controls.TextBlock]@{ Text = $text; FontSize = 12.5; Margin = '7,0,0,0' })
    $b.Child = $sp; $b
}
# שורה בכרטיס: אייקון בעיגול, כותרת ותיאור. מחזיר את האייקון כדי לעדכן מצב.
function New-Row($panel, $code, $title, $desc, [switch]$Last) {
    $g = [Windows.Controls.Grid]@{ Margin = '14,12,14,12' }
    [void]$g.ColumnDefinitions.Add([Windows.Controls.ColumnDefinition]@{ Width = 'Auto' })
    [void]$g.ColumnDefinitions.Add([Windows.Controls.ColumnDefinition]@{})
    $disc = [Windows.Controls.Border]@{ Width = 36; Height = 36; CornerRadius = 18; VerticalAlignment = 'Top' }
    $disc.Background = Brush $T.ACCENTSOFT
    $ic = New-Glyph $code 16 $T.ACCENT; $ic.HorizontalAlignment = 'Center'
    $ic.RenderTransformOrigin = '0.5,0.5'; $ic.RenderTransform = [Windows.Media.RotateTransform]::new()
    $disc.Child = $ic
    $tx = [Windows.Controls.StackPanel]@{ Margin = '14,0,0,0'; VerticalAlignment = 'Center' }
    [void]$tx.Children.Add([Windows.Controls.TextBlock]@{ Text = $title; FontWeight = 'SemiBold' })
    $d = [Windows.Controls.TextBlock]@{ Text = $desc; TextWrapping = 'Wrap'; FontSize = 12.5; Margin = '0,2,0,0' }
    $d.Foreground = Brush $T.FG2
    [void]$tx.Children.Add($d)
    [Windows.Controls.Grid]::SetColumn($tx, 1)
    [void]$g.Children.Add($disc); [void]$g.Children.Add($tx)
    [void]$panel.Children.Add($g)
    if (-not $Last) {
        $sep = [Windows.Controls.Border]@{ Height = 1; Margin = '64,0,14,0' }; $sep.Background = Brush $T.CARDBORDER
        [void]$panel.Children.Add($sep)
    }
    @{ Disc = $disc; Icon = $ic; Desc = $d; Code = $code }
}
function Set-RowState($row, $state, $note) {
    $spin = $row.Icon.RenderTransform
    $spin.BeginAnimation([Windows.Media.RotateTransform]::AngleProperty, $null)
    switch ($state) {
        'run'  { $row.Icon.Text = [string][char]0xE895; $row.Icon.Foreground = Brush $T.ACCENT; $row.Disc.Background = Brush $T.ACCENTSOFT
                 $a = [Windows.Media.Animation.DoubleAnimation]::new(0, 360, [Windows.Duration]::new([TimeSpan]::FromSeconds(1.1)))
                 $a.RepeatBehavior = [Windows.Media.Animation.RepeatBehavior]::Forever
                 $spin.BeginAnimation([Windows.Media.RotateTransform]::AngleProperty, $a) }
        'ok'   { $row.Icon.Text = [string][char]0xE73E; $row.Icon.Foreground = Brush '#FFFFFF'; $row.Disc.Background = Brush $T.GOOD }
        'skip' { $row.Icon.Text = [string][char]0xE73E; $row.Icon.Foreground = Brush $T.GOOD; $row.Disc.Background = Brush $T.ACCENTSOFT }
        'fail' { $row.Icon.Text = [string][char]0xE711; $row.Icon.Foreground = Brush '#FFFFFF'; $row.Disc.Background = Brush $T.BAD }
    }
    if ($note) { $row.Desc.Text = $note }
}
function Show-Page($name) { foreach ($p in 'PageHome','PageUac','PageWork','PageDone') { $ui[$p].Visibility = if ($p -eq $name) { 'Visible' } else { 'Collapsed' } } }

# ---------- מצב המחשב ----------
$State = @{}
# The widgets button on the taskbar (TaskbarDa: 0 = off; missing = on) and the policy that turns widgets off.
$TaskbarKey = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced'
function Test-TaskbarWidgets { (Get-ItemProperty $TaskbarKey -Name TaskbarDa -ErrorAction SilentlyContinue).TaskbarDa -ne 0 }
function Test-WidgetsPolicyOff { (Get-ItemProperty 'HKLM:\SOFTWARE\Policies\Microsoft\Dsh' -Name AllowNewsAndInterests -ErrorAction SilentlyContinue).AllowNewsAndInterests -eq 0 }
function Read-State {
    $State.Win11   = $build -ge 22621
    $State.Widgets = [bool](Get-AppxPackage MicrosoftWindows.Client.WebExperience -ErrorAction SilentlyContinue)
    $State.Taskbar = Test-TaskbarWidgets
    $State.PolicyOff = Test-WidgetsPolicyOff
    $State.Runtime = [bool](Get-Runtime)
    $State.Open    = Test-PolicyOpen
    $State.Trusted = Test-Trusted
    $State.Pkg     = Get-AppxPackage $PkgName -ErrorAction SilentlyContinue
}

function Show-Home {
    Read-State
    Show-Page 'PageHome'
    $ui.BtnPrimary.Visibility = 'Visible'
    $ui.Chips.Children.Clear(); $ui.Plan.Children.Clear()
    [void]$ui.Chips.Children.Add((New-Chip $State.Win11 $(if ($State.Win11) { 'Windows 11' } else { 'נדרש Windows 11 22H2 ומעלה' })))
    [void]$ui.Chips.Children.Add((New-Chip $State.Widgets $(if ($State.Widgets) { "לוח הווידג'טים מותקן" } else { "לוח הווידג'טים חסר" })))
    if ($State.PolicyOff) { [void]$ui.Chips.Children.Add((New-Chip $false "מדיניות במחשב מכבה את הווידג'טים")) }
    else { [void]$ui.Chips.Children.Add((New-Chip $(if ($State.Taskbar) { $true } else { $null }) $(if ($State.Taskbar) { "הכפתור בשורת המשימות דלוק" } else { "הכפתור בשורת המשימות כבוי, יודלק" }))) }
    [void]$ui.Chips.Children.Add((New-Chip $(if ($State.Open) { $true } else { $null }) $(if ($State.Open) { 'פידים של צד שלישי פתוחים' } else { 'פידים של צד שלישי סגורים' })))
    if ($State.Pkg) { [void]$ui.Chips.Children.Add((New-Chip $true "מותקנת גרסה $($State.Pkg.Version)")) }
    if ($script:Latest) { [void]$ui.Chips.Children.Add((New-Chip $true "גרסה חדשה יותר ב-GitHub: $($script:Latest)")) }

        $prep = 'מיקרוסופט מאפשרת להחליף את החדשות בלוח רק באירופה. המתקין מוסיף את האזור שלך לשלושה כללים בקובץ ההגדרות האזוריות של Windows (ושומר עותק של המקור), ומאשר את החתימה של הפיד. בונוס: קישורים מהלוח ייפתחו בדפדפן שלך ולא ב-Edge.'
    [void](New-Row $ui.Plan 0xE7EF 'הכנת Windows' $(if ($State.Open -and $State.Trusted) { 'כבר מוכן במחשב הזה.' } else { "$prep תופיע בקשה לאישור מנהל." }))
    [void](New-Row $ui.Plan 0xE896 'Windows App Runtime' $(if ($State.Runtime) { 'רכיב של מיקרוסופט שהפיד צריך. כבר מותקן.' } else { 'רכיב של מיקרוסופט שהפיד צריך. יורד ישירות ממיקרוסופט (כ-60MB).' }))
    $feed = 'רכיב קטן שמציג בלוח את הידיעות, התמונות והסרטונים, ומוסיף ווידג''טים והתראות. צורך כ-5MB זיכרון.'
    if ($script:Latest) { $feed += " יש ב-GitHub גרסה חדשה יותר ($($script:Latest)), והיא זו שתותקן. אחר כך הפיד מתעדכן לבד, בלי לעשות כלום." }
    [void](New-Row $ui.Plan 0xE789 'הפיד' $feed -Last)

    $ui.HomeNote.Text = 'אחרי עדכון גדול של Windows, ייתכן שהקובץ יחזור למקור והפיד ייעלם מהלוח. אם זה קורה, מריצים את הקובץ הזה שוב.'
    $ui.BtnPrimary.IsEnabled = $State.Win11 -and $State.Widgets -and -not $State.PolicyOff
    $ui.BtnPrimary.Content = if (-not $State.Pkg) { 'התקנה' } elseif (-not $State.Open) { 'תיקון' } else { 'התקנה מחדש' }
    $ui.BtnSecondary.Visibility = if ($State.Pkg -or (Test-Path "$Policy.orig")) { 'Visible' } else { 'Collapsed' }
    $ui.BtnSecondary.Content = 'הסרה'
    $ui.BtnClose.Content = 'סגירה'
}

# ---------- עבודה ברקע ----------
# הצעדים רצים ב-runspace נפרד כדי שהחלון לא ייתקע. הממשק קורא את $Sync בטיימר.
$Sync = [hashtable]::Synchronized(@{})
$Worker = {
    param($Sync, $Self, $PkgName, $Mode)
    function Step($i, $state, $note) { $Sync["s$i"] = @($state, $note) }
    # The temp folder by its full name. %TEMP% can hold the short (8.3) name of a Hebrew user folder, which
    # PowerShell's path commands do not find; the known-folder API gives the long one.
    $Tmp = Join-Path ([Environment]::GetFolderPath('LocalApplicationData')) 'Temp'
    [void][IO.Directory]::CreateDirectory($Tmp)
    Set-Location -LiteralPath $Tmp
    # The admin step, with no window: conhost --headless runs PowerShell (Windows names it "Console Window Host"
    # in the request, as the explanation page shows). The step writes its result to a file.
    function Elevated($arg) {
        $out = "$env:ProgramData\HagizraFeed\step-$([guid]::NewGuid().ToString('N')).txt"
        try { Start-Process conhost.exe -ArgumentList "--headless powershell.exe -NoProfile -ExecutionPolicy Bypass -File `"$env:HZ_PS1`" $arg `"$out`" `"$Self`"" -Verb RunAs -Wait | Out-Null }
        catch { return -1 }   # המשתמש סירב לבקשת המנהל
        if (Test-Path $out) { [int](Get-Content $out -Raw) } else { 10 }
    }
    try {
        if ($Mode -eq 'install') {
            Step 0 'run'
            $c = if ($Sync.Open -and $Sync.Trusted) { 1 } else { Elevated 'setup' }   # כבר מוכן: בלי בקשת מנהל
            if ($c -eq -1) { Step 0 'fail' 'לא ניתן אישור מנהל. בלי זה Windows לא יציג את הפיד.'; throw 'stop' }
            if ($c -ge 10) { Step 0 'fail' "השינוי נכשל. פרטים ביומן: $env:ProgramData\HagizraFeed\setup.log"; throw 'stop' }
            $Sync.Restart = ($c -eq 0)
            Step 0 $(if ($c -eq 0) { 'ok' } else { 'skip' }) $(if ($c -eq 0) { 'מוכן. עותק של הקובץ המקורי נשמר לידו.' } else { 'כבר מוכן במחשב הזה.' })

            Step 1 'run'
            if (Get-Runtime) { Step 1 'skip' 'כבר מותקן.' }
            else {
                $exe = Join-Path $Tmp 'WindowsAppRuntimeInstall-x64.exe'
                Step 1 'run' 'מוריד ממיקרוסופט...'
                curl.exe -sSfL -o $exe 'https://aka.ms/windowsappsdk/1.6/latest/windowsappruntimeinstall-x64.exe'
                if ($LASTEXITCODE -or -not (Test-Path $exe)) { Step 1 'fail' 'ההורדה נכשלה. בדוק את החיבור לאינטרנט ונסה שוב.'; throw 'stop' }
                Step 1 'run' 'מתקין...'
                # .NET, not Start-Process: Start-Process checks PowerShell's current folder, and the short (8.3) name
                # of a Hebrew user folder (C:\Users\870E~1) fails that check.
                $psi = New-Object Diagnostics.ProcessStartInfo $exe, '--quiet'
                $psi.UseShellExecute = $false; $psi.CreateNoWindow = $true; $psi.WorkingDirectory = $Tmp
                [Diagnostics.Process]::Start($psi).WaitForExit()
                Remove-Item $exe -Force -ErrorAction SilentlyContinue
                if (-not (Get-Runtime)) { Step 1 'fail' 'ההתקנה של הרכיב נכשלה.'; throw 'stop' }
                Step 1 'ok' 'הותקן.'
            }

            Step 2 'run'
            # A newer release on GitHub wins over the package inside this file. It is signed with the same
            # certificate, so Windows accepts it; anything else fails, and the package inside is installed instead.
            $done = $null
            try {
                $rel = Invoke-RestMethod 'https://api.github.com/repos/The-Young-boy/HaFeed/releases/latest' -Headers @{ 'User-Agent' = 'HaFeed-Setup' } -TimeoutSec 15
                $tag = [version]($rel.tag_name.TrimStart('v'))
                $asset = $rel.assets | Where-Object name -eq 'HaFeed.msix' | Select-Object -First 1
                if ($asset -and $tag -gt [version]$Version) {
                    Step 2 'run' "מוריד את גרסה $tag מ-GitHub..."
                    $newer = Join-Path $Tmp "$PkgName-$tag.msix"
                    curl.exe -sSfL -o $newer $asset.browser_download_url
                    if (-not $LASTEXITCODE -and (Test-Path $newer)) {
                        try { Add-AppxPackage -Path $newer -ForceApplicationShutdown -ForceUpdateFromAnyVersion -ErrorAction Stop; $done = "$tag" } catch { }
                    }
                    Remove-Item $newer -Force -ErrorAction SilentlyContinue
                }
            } catch { }
            if (-not $done) {
                $msix = Join-Path $Tmp "$PkgName.msix"
                [IO.File]::WriteAllBytes($msix, (Get-Data 'MSIX'))
                try { Add-AppxPackage -Path $msix -AllowUnsigned -ForceApplicationShutdown -ForceUpdateFromAnyVersion -ErrorAction Stop; $done = $Version }
                catch { Step 2 'fail' "ההתקנה נכשלה: $(($_.Exception.Message -split "`r?`n")[0])"; throw 'stop' }
                finally { Remove-Item $msix -Force -ErrorAction SilentlyContinue }
            }
            Step 2 'ok' "גרסה $done מותקנת. מעכשיו היא מתעדכנת לבד."
            # The widgets button on the taskbar. Newer Windows protects this value from scripts (UCPD);
            # then the done page opens the taskbar settings instead of working around it.
            $k = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced'
            if ((Get-ItemProperty $k -Name TaskbarDa -ErrorAction SilentlyContinue).TaskbarDa -eq 0) {
                try { Set-ItemProperty $k -Name TaskbarDa -Value 1 -Type DWord -ErrorAction Stop } catch { }
                $Sync.TaskbarOff = (Get-ItemProperty $k -Name TaskbarDa -ErrorAction SilentlyContinue).TaskbarDa -eq 0
            }
            # הלוח קורא את רשימת הפידים מחדש כשהוא עולה.
            Get-Process Widgets, WidgetBoard -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
        } else {
            Step 0 'run'
            $p = Get-AppxPackage $PkgName
            if ($p) { try { $p | Remove-AppxPackage -ErrorAction Stop } catch { Step 0 'fail' "ההסרה נכשלה: $(($_.Exception.Message -split "`r?`n")[0])"; throw 'stop' } }
            Step 0 'ok' $(if ($p) { 'הוסר.' } else { 'לא היה מותקן.' })
            Step 1 'run'
            $c = if ($Sync.HasBackup -or $Sync.Trusted) { Elevated 'remove' } else { 3 }
            if ($c -eq -1) { Step 1 'fail' 'לא ניתן אישור מנהל. הקובץ נשאר פתוח.'; throw 'stop' }
            if ($c -ge 10) { Step 1 'fail' "השחזור נכשל. פרטים ביומן: $env:ProgramData\HagizraFeed\setup.log"; throw 'stop' }
            $Sync.Restart = ($c -eq 2)
            Step 1 $(if ($c -eq 2) { 'ok' } else { 'skip' }) $(if ($c -eq 2) { 'הקובץ המקורי הוחזר, והתעודה הוסרה.' } else { 'לא נשאר מה להחזיר.' })
        }
        $Sync.Result = 'ok'
    } catch { $Sync.Result = 'fail'; if ("$_" -ne 'stop') { $Sync.Error = "$_" } }
}

function Start-Work($mode) {
    $Sync.Clear()
    $Sync.Open = Test-PolicyOpen; $Sync.HasBackup = Test-Path "$Policy.orig"; $Sync.Trusted = Test-Trusted
    Show-Page 'PageWork'
    $ui.Steps.Children.Clear(); $ui.WorkError.Visibility = 'Collapsed'; $ui.Prog.Value = 0
    $ui.BtnPrimary.Visibility = 'Collapsed'; $ui.BtnSecondary.Visibility = 'Collapsed'; $ui.BtnClose.IsEnabled = $false
    $rows = if ($mode -eq 'install') {
        $ui.WorkTitle.Text = 'מתקין את הפיד'
        (New-Row $ui.Steps 0xE7EF 'הכנת Windows' 'ממתין לאישור מנהל...'),
        (New-Row $ui.Steps 0xE896 'Windows App Runtime' 'ממתין...'),
        (New-Row $ui.Steps 0xE789 'הפיד' 'ממתין...' -Last)
    } else {
        $ui.WorkTitle.Text = 'מסיר את הפיד'
        (New-Row $ui.Steps 0xE74D 'הסרת הפיד' 'ממתין...'),
        (New-Row $ui.Steps 0xE777 'החזרת Windows למצב המקורי' 'ממתין לאישור מנהל...' -Last)
    }
    $ps = [PowerShell]::Create()
    # הפונקציות של המתקין לא קיימות ב-runspace החדש, ולכן מגדירים שם את מה שצריך.
    $defs = "function Get-Data { ${function:Get-Data} }`nfunction Test-Trusted { ${function:Test-Trusted} }`n`$CertThumb = '$CertThumb'`n`$Version = '$Version'`nfunction Get-Runtime { ${function:Get-Runtime} }`n`$Self = '$($Self -replace "'", "''")'"
    $ps.AddScript($defs).AddStatement() | Out-Null
    $ps.AddScript($Worker.ToString()).AddArgument($Sync).AddArgument($Self).AddArgument($PkgName).AddArgument($mode) | Out-Null
    $script:Job = @{ Ps = $ps; Handle = $ps.BeginInvoke(); Rows = $rows; Mode = $mode }
    $timer.Start()
}

$timer = [Windows.Threading.DispatcherTimer]@{ Interval = [TimeSpan]::FromMilliseconds(120) }
$timer.Add_Tick({
    try {
        $j = $script:Job; $done = 0
        for ($i = 0; $i -lt $j.Rows.Count; $i++) {
            $s = $Sync["s$i"]
            if ($s -and "$($s[0])|$($s[1])" -ne $j["last$i"]) { Set-RowState $j.Rows[$i] $s[0] $s[1]; $j["last$i"] = "$($s[0])|$($s[1])" }
            if ($s -and $s[0] -in 'ok', 'skip') { $done++ }
        }
        $target = 100 * $done / $j.Rows.Count
        $ui.Prog.Value = [Math]::Max($ui.Prog.Value, $target)
        if (-not $j.Handle.IsCompleted) { return }
        $timer.Stop(); $j.Ps.Dispose()
        Show-Done $j.Mode
    } catch { }
})

function Add-GuideStep($n, $title, $desc) {
    $b = [Windows.Controls.Border]@{ Margin = '0,0,0,8'; Padding = '14,12' }; $b.Style = $win.FindResource('Card')
    $g = [Windows.Controls.Grid]@{}
    [void]$g.ColumnDefinitions.Add([Windows.Controls.ColumnDefinition]@{ Width = 'Auto' })
    [void]$g.ColumnDefinitions.Add([Windows.Controls.ColumnDefinition]@{})
    $num = [Windows.Controls.Border]@{ Width = 26; Height = 26; CornerRadius = 13; VerticalAlignment = 'Top' }; $num.Background = Brush $T.ACCENT
    $nt = [Windows.Controls.TextBlock]@{ Text = "$n"; FontWeight = 'Bold'; FontSize = 13; HorizontalAlignment = 'Center'; VerticalAlignment = 'Center' }
    $nt.Foreground = Brush $T.ONACCENT; $num.Child = $nt
    $tx = [Windows.Controls.StackPanel]@{ Margin = '12,2,0,0' }
    [void]$tx.Children.Add([Windows.Controls.TextBlock]@{ Text = $title; FontWeight = 'SemiBold' })
    if ($desc) { $d = [Windows.Controls.TextBlock]@{ Text = $desc; TextWrapping = 'Wrap'; FontSize = 12.5; Margin = '0,2,0,0' }; $d.Foreground = Brush $T.FG2; [void]$tx.Children.Add($d) }
    [Windows.Controls.Grid]::SetColumn($tx, 1)
    [void]$g.Children.Add($num); [void]$g.Children.Add($tx)
    $b.Child = $g; [void]$ui.Guide.Children.Add($b)
}

function Show-Done($mode) {
    $ui.BtnClose.IsEnabled = $true; $ui.BtnPrimary.Visibility = 'Visible'
    if ($Sync.Result -ne 'ok') {
        # נשארים במסך העבודה: השלב שנכשל מסומן באדום, עם ההסבר שלו.
        $ui.WorkTitle.Text = if ($mode -eq 'install') { 'ההתקנה לא הושלמה' } else { 'ההסרה לא הושלמה' }
        if ($Sync.Error) { $ui.WorkError.Text = $Sync.Error; $ui.WorkError.Visibility = 'Visible' }
        $ui.BtnPrimary.Content = 'חזרה'; $script:Next = 'home'
        return
    }
    Show-Page 'PageDone'
    $ui.Guide.Children.Clear()
    $restart = [bool]$Sync.Restart
    if ($mode -eq 'install') {
        $ui.DoneTitle.Text = 'הפיד מותקן'
        $ui.DoneText.Text = 'נשארו כמה צעדים קצרים, ואז הידיעות יופיעו בלוח.'
        $n = 1
        if ($restart) { Add-GuideStep ($n++) 'הפעלה מחדש של המחשב' 'Windows קורא את ההגדרות האזוריות רק בעלייה.' }
        if ($Sync.TaskbarOff) {
            Add-GuideStep ($n++) "מדליקים את כפתור הווידג'טים" "Windows לא נותן לתוכנות להדליק אותו. בחלון ההגדרות שנפתח, מדליקים את המתג ""ווידג'טים"" תחת ""פריטים בשורת המשימות""."
            Start-Process 'ms-settings:taskbar'
        }
        Add-GuideStep ($n++) "פותחים את הווידג'טים" 'מקש Windows + W, או הכפתור בשורת המשימות.'
        Add-GuideStep ($n++) 'בודקים שהמתג של "הפיד" דלוק' 'גלגל השיניים בפינת הלוח. בדרך כלל הוא נדלק לבד. באותו מקום אפשר לכבות את "גלה", החדשות של מיקרוסופט.'
        Add-GuideStep ($n++) 'לוחצים על הלוגו של הפיד' 'בסרגל הצד של הלוח. בפתיחה הראשונה עולה אשף קצר לבחירת הערוצים.'
        Add-GuideStep ($n++) "רוצים גם ווידג'טים?" "בלוח לוחצים על + , בוחרים ""הפיד"", ומוסיפים מבזקים, ערוץ או פיד, או כותרת ראשית."
        $ui.BtnPrimary.Content = if ($restart) { 'הפעלה מחדש עכשיו' } else { "פתיחת הווידג'טים" }
        $script:Next = if ($restart) { 'restart' } else { 'widgets' }
    } else {
        $ui.DoneTitle.Text = 'הפיד הוסר'
        $ui.DoneText.Text = if ($restart) { 'קובץ ההגדרות האזוריות חזר למקור. אחרי הפעלה מחדש, הלוח יחזור להיות כמו שהיה.' } else { 'לא נשאר במחשב שום דבר מהפיד.' }
        $ui.BtnPrimary.Content = if ($restart) { 'הפעלה מחדש עכשיו' } else { 'סגירה' }
        $script:Next = if ($restart) { 'restart' } else { 'close' }
    }
    $ui.BtnClose.Content = if ($restart) { 'אחר כך' } else { 'סגירה' }
}

# ---------- לפני בקשת המנהל ----------
# Windows asks only when it is needed; before that, a page says what will appear and where to click.
function Show-Uac($mode) {
    Show-Page 'PageUac'
    $ui.UacText.Text = if ($mode -eq 'install') {
        'כדי שהפיד יופיע בלוח, ההתקנה משנה קובץ הגדרות אחד של Windows ומאשרת את החתימה של הפיד. בשביל זה Windows מבקש אישור מנהל, פעם אחת. אחרי "המשך" יופיע חלון כמו בדוגמה. לוחצים "כן". השם בחלון הוא Console Window Host, רכיב של Windows שמבצע את השינוי בלי להציג חלון.'
    } else {
        'כדי להחזיר את Windows למצב המקורי, ההסרה מחזירה את קובץ ההגדרות ומוחקת את החתימה של הפיד. בשביל זה Windows מבקש אישור מנהל, פעם אחת. אחרי "המשך" יופיע חלון כמו בדוגמה. לוחצים "כן". השם בחלון הוא Console Window Host, רכיב של Windows שמבצע את השינוי בלי להציג חלון.'
    }
    $a = [Windows.Media.Animation.DoubleAnimation]::new(1, 0.25, [Windows.Duration]::new([TimeSpan]::FromSeconds(0.8)))
    $a.AutoReverse = $true; $a.RepeatBehavior = [Windows.Media.Animation.RepeatBehavior]::Forever
    $ui.UacRing.BeginAnimation([Windows.UIElement]::OpacityProperty, $a)
    $ui.BtnPrimary.Content = 'המשך'; $ui.BtnPrimary.Visibility = 'Visible'
    $ui.BtnSecondary.Visibility = 'Collapsed'
    $ui.BtnClose.Content = 'חזרה'
    $script:Next = 'elevate'; $script:After = $mode
}

# ---------- כפתורים ----------
$script:Next = 'install'
$ui.BtnPrimary.Add_Click({
    switch ($script:Next) {
        'install' { if ($State.Open -and $State.Trusted) { Start-Work 'install' } else { Show-Uac 'install' } }
        'elevate' { $script:Next = 'install'; Start-Work $script:After }
        'home'    { $script:Next = 'install'; Show-Home }
        'restart' { shutdown.exe /r /t 5; $win.Close() }
        'widgets' { Start-Process 'ms-widgets:'; $win.Close() }
        'close'   { $win.Close() }
    }
})
$ui.BtnSecondary.Add_Click({ if ((Test-Path "$Policy.orig") -or $State.Trusted) { Show-Uac 'uninstall' } else { Start-Work 'uninstall' } })
$ui.BtnClose.Add_Click({ if ($script:Next -eq 'elevate') { $script:Next = 'install'; Show-Home } else { $win.Close() } })

# ---------- /quiet: כל ההתקנה בלי חלון, לטכנאים ----------
# The same steps as the button, in this process; the window is built above but never shown.
if ($env:HZ_MODE -eq 'quiet') {
    $Sync.Clear(); $Sync.Open = Test-PolicyOpen; $Sync.HasBackup = Test-Path "$Policy.orig"; $Sync.Trusted = Test-Trusted
    & $Worker $Sync $Self $PkgName 'install'
    Write-Log "quiet install: $($Sync.Result) $($Sync.Error)"
    exit $(if ($Sync.Result -ne 'ok') { 1 } elseif ($Sync.Restart) { 3010 } else { 0 })
}

# ---------- גרסה חדשה יותר ב-GitHub ----------
# Asked beside the window (it may take seconds); when there is one, the home page says it is the one installed.
$script:Latest = $null
$LatestPs = [PowerShell]::Create()
[void]$LatestPs.AddScript({ param($v)
    try {
        $r = Invoke-RestMethod 'https://api.github.com/repos/The-Young-boy/HaFeed/releases/latest' -Headers @{ 'User-Agent' = 'HaFeed-Setup' } -TimeoutSec 15
        $t = [version]($r.tag_name.TrimStart('v'))
        if (($r.assets | Where-Object name -eq 'HaFeed.msix') -and $t -gt [version]$v) { "$t" }
    } catch { }
}).AddArgument($Version)
$LatestRun = $LatestPs.BeginInvoke()
$LatestTimer = New-Object Windows.Threading.DispatcherTimer
$LatestTimer.Interval = [TimeSpan]::FromMilliseconds(400)
$LatestTimer.Add_Tick({
    if (-not $LatestRun.IsCompleted) { return }
    $LatestTimer.Stop()
    $script:Latest = @($LatestPs.EndInvoke($LatestRun))[0]
    if ($script:Latest -and $ui.PageHome.Visibility -eq 'Visible') { Show-Home }
})
$LatestTimer.Start()

Write-Log "window opened ($Version)"
$win.Add_ContentRendered({ [void]$win.Activate() })   # in front, also when started from a hidden process
Show-Home
[void]$win.ShowDialog()
