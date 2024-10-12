@echo off
setlocal

echo Starting server using Windows

echo Installing dependencies for Rust...

:: Check if rustup is installed
where rustup >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo rustup not found, installing...
    powershell -Command "Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe"
    rustup-init.exe -y
    del rustup-init.exe
) else (
    rustup update
)

echo Building backend...
cargo build --release

echo Downloading Server Deps...

:: Check if Node.js is installed
where node >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Node.js not found, installing...
    powershell -Command "Invoke-WebRequest -Uri https://nodejs.org/dist/v20.0.0/node-v20.0.0-x64.msi -OutFile nodejs.msi"
    msiexec /i nodejs.msi /quiet /norestart
    del nodejs.msi
) 

echo Downloading npm dependencies...
npm install

echo Starting backend and server...

:: Run backend and server simultaneously
start "" cargo run --release
start "" npm run dev

endlocal