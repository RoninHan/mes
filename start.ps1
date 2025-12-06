# MES 系统一键启动脚本 (Windows PowerShell)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  MES 系统一键启动脚本" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 检查 Docker 是否安装
Write-Host "[1/5] 检查 Docker 环境..." -ForegroundColor Yellow
try {
    $dockerVersion = docker --version
    Write-Host "✓ Docker 已安装: $dockerVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ Docker 未安装，请先安装 Docker Desktop" -ForegroundColor Red
    exit 1
}

# 检查 Docker Compose 是否安装
try {
    $composeVersion = docker compose version
    Write-Host "✓ Docker Compose 已安装: $composeVersion" -ForegroundColor Green
} catch {
    Write-Host "✗ Docker Compose 未安装" -ForegroundColor Red
    exit 1
}

# 检查 Docker 是否运行
try {
    docker info | Out-Null
    Write-Host "✓ Docker 服务正在运行" -ForegroundColor Green
} catch {
    Write-Host "✗ Docker 服务未运行，请启动 Docker Desktop" -ForegroundColor Red
    exit 1
}

Write-Host ""

# 检查 .env 文件
Write-Host "[2/5] 检查环境配置..." -ForegroundColor Yellow
if (-Not (Test-Path ".env")) {
    Write-Host "⚠ .env 文件不存在，从 .env.example 创建..." -ForegroundColor Yellow
    if (Test-Path ".env.example") {
        Copy-Item ".env.example" ".env"
        Write-Host "✓ 已创建 .env 文件，请根据需要修改配置" -ForegroundColor Green
    } else {
        Write-Host "⚠ .env.example 不存在，使用默认配置" -ForegroundColor Yellow
    }
} else {
    Write-Host "✓ .env 文件已存在" -ForegroundColor Green
}

Write-Host ""

# 停止现有容器（如果有）
Write-Host "[3/5] 停止现有容器..." -ForegroundColor Yellow
docker compose down 2>&1 | Out-Null
Write-Host "✓ 已停止现有容器" -ForegroundColor Green

Write-Host ""

# 构建镜像
Write-Host "[4/5] 构建 Docker 镜像..." -ForegroundColor Yellow
docker compose build --no-cache
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ 镜像构建失败" -ForegroundColor Red
    exit 1
}
Write-Host "✓ 镜像构建完成" -ForegroundColor Green

Write-Host ""

# 启动服务
Write-Host "[5/5] 启动服务..." -ForegroundColor Yellow
docker compose up -d
if ($LASTEXITCODE -ne 0) {
    Write-Host "✗ 服务启动失败" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  服务启动成功！" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "访问地址：" -ForegroundColor Yellow
Write-Host "  前端: http://localhost" -ForegroundColor White
Write-Host "  后端 API: http://localhost:8080" -ForegroundColor White
Write-Host "  健康检查: http://localhost:8080/health" -ForegroundColor White
Write-Host ""
Write-Host "查看日志：" -ForegroundColor Yellow
Write-Host "  docker compose logs -f" -ForegroundColor White
Write-Host ""
Write-Host "停止服务：" -ForegroundColor Yellow
Write-Host "  docker compose down" -ForegroundColor White
Write-Host ""

# 等待服务就绪
Write-Host "等待服务就绪..." -ForegroundColor Yellow
Start-Sleep -Seconds 5

# 检查后端健康状态
$maxRetries = 30
$retryCount = 0
$backendReady = $false

while ($retryCount -lt $maxRetries -and -not $backendReady) {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -TimeoutSec 2 -ErrorAction Stop
        if ($response.StatusCode -eq 200) {
            $backendReady = $true
            Write-Host "✓ 后端服务已就绪" -ForegroundColor Green
        }
    } catch {
        $retryCount++
        Write-Host "." -NoNewline -ForegroundColor Gray
        Start-Sleep -Seconds 2
    }
}

if (-not $backendReady) {
    Write-Host ""
    Write-Host "⚠ 后端服务可能尚未完全启动，请稍后访问" -ForegroundColor Yellow
}

Write-Host ""


