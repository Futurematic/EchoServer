param (
    [int]$Port = 1234,
    [string]$LogFile = ".\echo-$(Get-Date -Format 'yyyy-MM-dd_HH-mm-ss').log"
)

$listener = [System.Net.Sockets.TcpListener]::new($Port)
$listener.Start()
Write-Host "🔁 Echo-Server läuft auf Port $Port"
Write-Host "📁 Logging nach: $LogFile`n"

Add-Content -Path $LogFile -Value "🔁 Echo-Server gestartet am $(Get-Date) auf Port $Port"

while ($true) {
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
            $writer.WriteLine($line)
        }

        $client.Close()
        $closeTime = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        Write-Host "🔌 Verbindung von ${clientIP} beendet"
        Add-Content -Path $LogFile -Value "[$closeTime] Verbindung zu ${clientIP} beendet`n"
    }
    catch {
        $errorTime = Get-Date -Format "HH:mm:ss"
        Write-Host "⚠️  Fehler: $($_.Exception.Message)"
        Add-Content -Path $LogFile -Value "[$errorTime] ⚠️ Fehler: $($_.Exception.Message)"
        # Optional: kleine Pause, damit es nicht direkt wieder passiert
        Start-Sleep -Milliseconds 500
    }
}
