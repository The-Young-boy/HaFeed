# The update the user asked for (the button in About): a small window that cannot be closed while the
# package installs. It runs from a one-time scheduled task, outside the package that is being replaced.
# update.rs fills in {MSIX} {LOG} {LOGO} {VER} {TASK}. A background update does not use this file: it is silent.
$ProgressPreference = 'SilentlyContinue'
Add-Type -AssemblyName PresentationFramework
$light = (Get-ItemProperty 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize' -ErrorAction SilentlyContinue).AppsUseLightTheme -eq 1
$c = if ($light) { @{ bg = '#FBFBFB'; line = '#E0E0E0'; text = '#1B1B1B'; sub = '#5F5F5F'; track = '#E6E6E6' } } else { @{ bg = '#202020'; line = '#3A3A3A'; text = '#FFFFFF'; sub = '#B0B0B0'; track = '#3A3A3A' } }
[xml]$xaml = @"
<Window xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation" Title="הפיד" Width="400" SizeToContent="Height"
        WindowStyle="None" ResizeMode="NoResize" AllowsTransparency="True" Background="Transparent" Topmost="True"
        WindowStartupLocation="CenterScreen" FlowDirection="RightToLeft" FontFamily="Segoe UI">
  <Border CornerRadius="12" Background="$($c.bg)" BorderBrush="$($c.line)" BorderThickness="1" Padding="24,22" Margin="12">
    <Border.Effect><DropShadowEffect BlurRadius="24" ShadowDepth="4" Opacity="0.3"/></Border.Effect>
    <StackPanel>
      <DockPanel>
        <Image Name="Logo" Width="44" Height="44" DockPanel.Dock="Left"/>
        <StackPanel Margin="14,0,0,0" VerticalAlignment="Center">
          <TextBlock Name="Head" Text="מעדכן את הפיד" FontSize="16" FontWeight="SemiBold" Foreground="$($c.text)"/>
          <TextBlock Name="Sub" Text="לגרסה {VER}. זה לוקח רגע, והפיד חוזר לבד." FontSize="13" Foreground="$($c.sub)" TextWrapping="Wrap" Margin="0,3,0,0"/>
        </StackPanel>
      </DockPanel>
      <ProgressBar Name="Bar" IsIndeterminate="True" Height="4" Margin="0,20,0,0" Foreground="#E5484D" Background="$($c.track)" BorderThickness="0"/>
    </StackPanel>
  </Border>
</Window>
"@
$w = [Windows.Markup.XamlReader]::Load((New-Object System.Xml.XmlNodeReader $xaml))
$f = @{}; foreach ($n in 'Logo', 'Head', 'Sub', 'Bar') { $f[$n] = $w.FindName($n) }
try { $f.Logo.Source = [Windows.Media.Imaging.BitmapFrame]::Create([Uri]'{LOGO}') } catch {}
# No close button, and Alt+F4 does nothing until the install ends.
$script:done = $false
$w.Add_Closing({ param($s, $e) if (-not $script:done) { $e.Cancel = $true } })
$w.Add_MouseLeftButtonDown({ try { $w.DragMove() } catch {} })

# The install runs beside the window, so the window keeps drawing.
$ps = [PowerShell]::Create()
[void]$ps.AddScript({ param($m) $ProgressPreference = 'SilentlyContinue'; Add-AppxPackage -Path $m -ForceApplicationShutdown -ErrorAction Stop }).AddArgument('{MSIX}')
$run = $ps.BeginInvoke()
$t = New-Object Windows.Threading.DispatcherTimer
$t.Interval = [TimeSpan]::FromMilliseconds(300)
$t.Add_Tick({
  if (-not $run.IsCompleted) { return }
  $t.Stop()
  $err = $null
  try { $ps.EndInvoke($run) | Out-Null } catch { $err = $_ }
  if (-not $err -and $ps.HadErrors) { $err = $ps.Streams.Error[0] }
  $f.Bar.IsIndeterminate = $false; $f.Bar.Value = 100
  if ($err) {
    $err | Out-File '{LOG}'
    $f.Head.Text = 'העדכון לא הצליח'
    $f.Sub.Text = 'הפיד נשאר בגרסה הקודמת. אפשר לנסות שוב מ"אודות".'
    $f.Bar.Foreground = '#C42B1C'
    $wait = 6
  } else {
    $f.Head.Text = 'הפיד עודכן'
    $f.Sub.Text = 'גרסה {VER} מותקנת.'
    $wait = 2
  }
  $script:done = $true
  $end = New-Object Windows.Threading.DispatcherTimer
  $end.Interval = [TimeSpan]::FromSeconds($wait)
  $end.Add_Tick({ $end.Stop(); $w.Close() })
  $end.Start()
})
$t.Start()
[void]$w.ShowDialog()
Remove-Item '{MSIX}' -Force -ErrorAction SilentlyContinue
schtasks.exe /delete /tn {TASK} /f | Out-Null
