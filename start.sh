#!/bin/bash

# MES 系统一键启动脚本 (Linux/Mac)

set -e

echo "========================================"
echo "  MES 系统一键启动脚本"
echo "========================================"
echo ""

# 检查 Docker 是否安装
echo "[1/5] 检查 Docker 环境..."
if ! command -v docker &> /dev/null; then
    echo "✗ Docker 未安装，请先安装 Docker"
    exit 1
fi
echo "✓ Docker 已安装: $(docker --version)"

if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo "✗ Docker Compose 未安装"
    exit 1
fi
echo "✓ Docker Compose 已安装"

# 检查 Docker 是否运行
if ! docker info &> /dev/null; then
    echo "✗ Docker 服务未运行，请启动 Docker"
    exit 1
fi
echo "✓ Docker 服务正在运行"

echo ""

# 检查 .env 文件
echo "[2/5] 检查环境配置..."
if [ ! -f .env ]; then
    echo "⚠ .env 文件不存在，从 .env.example 创建..."
    if [ -f .env.example ]; then
        cp .env.example .env
        echo "✓ 已创建 .env 文件，请根据需要修改配置"
    else
        echo "⚠ .env.example 不存在，使用默认配置"
    fi
else
    echo "✓ .env 文件已存在"
fi

echo ""

# 停止现有容器（如果有）
echo "[3/5] 停止现有容器..."
docker compose down 2>/dev/null || true
echo "✓ 已停止现有容器"

echo ""

# 构建镜像
echo "[4/5] 构建 Docker 镜像..."
docker compose build --no-cache
if [ $? -ne 0 ]; then
    echo "✗ 镜像构建失败"
    exit 1
fi
echo "✓ 镜像构建完成"

echo ""

# 启动服务
echo "[5/5] 启动服务..."
docker compose up -d
if [ $? -ne 0 ]; then
    echo "✗ 服务启动失败"
    exit 1
fi

echo ""
echo "========================================"
echo "  服务启动成功！"
echo "========================================"
echo ""
echo "访问地址："
echo "  前端: http://localhost"
echo "  后端 API: http://localhost:8080"
echo "  健康检查: http://localhost:8080/health"
echo ""
echo "查看日志："
echo "  docker compose logs -f"
echo ""
echo "停止服务："
echo "  docker compose down"
echo ""

# 等待服务就绪
echo "等待服务就绪..."
sleep 5

# 检查后端健康状态
max_retries=30
retry_count=0
backend_ready=false

while [ $retry_count -lt $max_retries ] && [ "$backend_ready" = false ]; do
    if curl -f -s http://localhost:8080/health > /dev/null 2>&1; then
        backend_ready=true
        echo "✓ 后端服务已就绪"
    else
        retry_count=$((retry_count + 1))
        echo -n "."
        sleep 2
    fi
done

if [ "$backend_ready" = false ]; then
    echo ""
    echo "⚠ 后端服务可能尚未完全启动，请稍后访问"
fi

echo ""


