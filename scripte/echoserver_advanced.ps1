param (
    [int]$Port = 1234,
    [string]$LogFile = ".\echo-$(Get-Date -Format 'yyyy-MM-dd_HH-mm-ss').log"
)

$listener = [System.Net.Sockets.TcpListener]::new($Port)
$listener.Start()
Write-Host "🔁 Echo-Server läuft auf Port $Port"
Write-Host "📁 Logging nach: $LogFile`n"

Add-Content -Path $LogFile -Value "🔁 Echo-Server gestartet am $(Get-Date) auf Port $Port"

$running = $true

while ($running) {
    try {
        $client = $listener.AcceptTcpClient()
        $stream = $client.GetStream()

        if (-not $stream) {
            throw "❌ Kein gültiger Netzwerkstream vom Client empfangen."
        }

        $reader = [System.IO.StreamReader]::new($stream)
        $writer = [System.IO.StreamWriter]::new($stream)
        $writer.AutoFlush = $true

        $clientIP = $client.Client.RemoteEndPoint.ToString()
        $connectTime = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        Write-Host "📡 Neue Verbindung von ${clientIP} @ $connectTime"
        Add-Content -Path $LogFile -Value "[$connectTime] Verbindung von $clientIP"

        while ($client.Connected -and !$reader.EndOfStream) {
            $line = $reader.ReadLine()
            $timestamp = Get-Date -Format "HH:mm:ss"
            Write-Host "🔹 ${clientIP}: $line"
            Add-Content -Path $LogFile -Value "[$timestamp] ${clientIP}: $line"

            if ($line -in @('exit', 'quit')) {
                $writer.WriteLine("👋 Server wird beendet. Bye!")
                $running = $false
                break
            } else {
                $writer.WriteLine($line)
            }
        }

        $client.Close()
        $closeTime = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        Write-Host "🔌 Verbindung von ${clientIP} beendet"
        Add-Content -Path $LogFile -Value "[$closeTime] Verbindung zu ${clientIP} beendet`n"
    }
    catch {
        $msg = $_.Exception.Message
        $errorTime = Get-Date -Format "HH:mm:ss"

        if ($msg -like "*softwaregesteuert*") {
            Write-Host "🔕 Verbindung wurde normal beendet vom Client"
            Add-Content -Path $LogFile -Value "[$errorTime] 🔕 Verbindung wurde durch Client beendet"
        } else {
            Write-Host "⚠️  Fehler: $msg"
            Add-Content -Path $LogFile -Value "[$errorTime] ⚠️ Fehler: $msg"
        }

        Start-Sleep -Milliseconds 500
    }
}

# Nach Verlassen der Hauptschleife
Write-Host "`n🛑 Server wird beendet..."
Add-Content -Path $LogFile -Value "`n🛑 Echo-Server wurde beendet am $(Get-Date)"
$listener.Stop()