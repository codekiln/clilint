# Installation

## Install with mise

If you use [mise](https://mise.jdx.dev/), install the latest Clilint release with:

```sh
mise use -g github:codekiln/clilint
```

Check the installation:

```sh
clilint --version
```

If your shell cannot find `clilint`, follow the [mise activation instructions](https://mise.jdx.dev/getting-started.html#activate-mise) and open a new terminal.

## Download Clilint

The [latest GitHub Release](https://github.com/codekiln/clilint/releases/latest) contains these downloads:

| System | Processor | File |
| --- | --- | --- |
| macOS | Apple silicon | `clilint-aarch64-apple-darwin.tar.xz` |
| macOS | Intel | `clilint-x86_64-apple-darwin.tar.xz` |
| Linux | x86-64 | `clilint-x86_64-unknown-linux-gnu.tar.xz` |
| Windows | x86-64 | `clilint-x86_64-pc-windows-msvc.zip` |

On macOS or Linux, `uname -m` prints the processor type. `arm64` or `aarch64` means Apple silicon; `x86_64` means Intel or AMD x86-64.

### macOS

Set `FILE` for your Mac:

```sh
# Apple silicon
FILE=clilint-aarch64-apple-darwin.tar.xz

# Intel: use this value instead
# FILE=clilint-x86_64-apple-darwin.tar.xz
```

Then run:

```sh
curl -LO "https://github.com/codekiln/clilint/releases/latest/download/$FILE"
curl -LO "https://github.com/codekiln/clilint/releases/latest/download/$FILE.sha256"
shasum -a 256 -c "$FILE.sha256"
tar -xJf "$FILE"
mkdir -p "$HOME/.local/bin"
install -m 0755 "${FILE%.tar.xz}/clilint" "$HOME/.local/bin/clilint"
```

Ensure `$HOME/.local/bin` is on `PATH`, then run `clilint --version`.

### Linux

The published Linux build requires an x86-64 GNU/Linux system. Run:

```sh
FILE=clilint-x86_64-unknown-linux-gnu.tar.xz
curl -LO "https://github.com/codekiln/clilint/releases/latest/download/$FILE"
curl -LO "https://github.com/codekiln/clilint/releases/latest/download/$FILE.sha256"
sha256sum -c "$FILE.sha256"
tar -xJf "$FILE"
mkdir -p "$HOME/.local/bin"
install -m 0755 "${FILE%.tar.xz}/clilint" "$HOME/.local/bin/clilint"
```

Ensure `$HOME/.local/bin` is on `PATH`, then run `clilint --version`.

### Windows

Open PowerShell and run:

```powershell
$File = "clilint-x86_64-pc-windows-msvc.zip"
$BaseUrl = "https://github.com/codekiln/clilint/releases/latest/download"
Invoke-WebRequest "$BaseUrl/$File" -OutFile $File
Invoke-WebRequest "$BaseUrl/$File.sha256" -OutFile "$File.sha256"

$Expected = (Get-Content "$File.sha256").Split()[0].ToLower()
$Actual = (Get-FileHash $File -Algorithm SHA256).Hash.ToLower()
if ($Actual -ne $Expected) { throw "Clilint download checksum did not match" }

Expand-Archive $File -DestinationPath clilint
New-Item -ItemType Directory -Force "$HOME\bin" | Out-Null
Copy-Item ".\clilint\clilint.exe" "$HOME\bin\clilint.exe"

$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$HOME\bin*") {
  [Environment]::SetEnvironmentVariable("Path", "$UserPath;$HOME\bin", "User")
}
$env:Path += ";$HOME\bin"
clilint --version
```

## Troubleshooting

- **`clilint: command not found`**: confirm the installation directory is on `PATH`, then open a new terminal.
- **No download matches your system**: build from source by following [Contributing](../CONTRIBUTING.md). The current release does not include Linux ARM or Windows ARM builds.
- **Checksum verification fails**: delete the download and checksum file, download both again from the same release, and retry. Do not run a file whose checksum does not match.
- **macOS blocks the executable**: confirm that the checksum matches the published file, then allow Clilint in **System Settings → Privacy & Security**.
