#!/usr/bin/env bash

# start-dev.sh — 本地开发模式：只用 Docker 启动数据库/依赖，前后端在本机运行
# 用法:
#   ./start-dev.sh                # 启动 postgres + redis（containers），并在本机运行 backend & frontend
#   ./start-dev.sh --no-frontend  # 不自动启动本地前端
#   ./start-dev.sh --no-backend   # 不自动启动本地后端

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"
MES_FRONTEND_PID=0

echo "========================================"
echo "  MES 本地开发一键启动脚本"
echo "========================================"
echo ""

# args
START_FRONTEND=true
START_BACKEND=true
for arg in "$@"; do
  case "$arg" in
    --no-frontend) START_FRONTEND=false ;;
    --no-backend)  START_BACKEND=false  ;;
    *) ;;
  esac
done

echo "[1/6] 检查 Docker 环境（用于运行数据库/依赖）..."
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
echo ""

echo "[2/6] 检查 .env 文件..."
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

echo "[3/6] 启动数据库与依赖（Postgres / Redis）..."
docker compose up -d postgres redis
echo "✓ 已请求启动 postgres & redis"

# 等待 Postgres 健康
echo "[4/6] 等待 Postgres 健康..."
max_retries=60
retry_count=0
until docker exec mes-postgres pg_isready -U mes_user -d mes_db >/dev/null 2>&1; do
  retry_count=$((retry_count + 1))
  if [ $retry_count -ge $max_retries ]; then
    echo ""
    echo "✗ Postgres 未能在超时内就绪"
    echo "查看容器日志： docker compose logs postgres"
    exit 1
  fi
  echo -n "."
  sleep 2
done
echo ""
echo "✓ Postgres 已就绪"

# 等待 Redis 健康 (简单 ping)
echo "[5/6] 等待 Redis 健康..."
retry_count=0
until docker exec mes-redis redis-cli ping >/dev/null 2>&1; do
  retry_count=$((retry_count + 1))
  if [ $retry_count -ge $max_retries ]; then
    echo ""
    echo "✗ Redis 未能在超时内就绪"
    echo "查看容器日志： docker compose logs redis"
    exit 1
  fi
  echo -n "."
  sleep 1
done
echo ""
echo "✓ Redis 已就绪"

echo ""
echo "[6/6] 在本机启动后端与前端（可选）..."

# backend
if [ "$START_BACKEND" = true ]; then
  if [ -d "./mes-backend" ]; then
    if command -v cargo &> /dev/null; then
      # check cargo version >= 1.85 (basic heuristic); if older, fallback to container
      CARGO_VER_RAW="$(cargo --version 2>/dev/null || true)"
      CARGO_VER="$(echo "$CARGO_VER_RAW" | awk '{print $2}' | cut -d. -f1,2)"
      echo "Detected cargo version: ${CARGO_VER:-unknown}"
      NEED_CONTAINER=false
      if [ -z "$CARGO_VER" ]; then
        NEED_CONTAINER=true
      else
        MAJOR="$(echo "$CARGO_VER" | cut -d. -f1)"
        MINOR="$(echo "$CARGO_VER" | cut -d. -f2)"
        if [ "$MAJOR" -lt 1 ] || { [ "$MAJOR" -eq 1 ] && [ "$MINOR" -lt 85 ]; }; then
          NEED_CONTAINER=true
        fi
      fi

      if [ "$NEED_CONTAINER" = true ]; then
        echo "⚠ 本机 cargo 版本较旧或未知（需要 >=1.85 才能编译部分依赖）。使用容器运行后端。"
        docker compose up -d backend
        echo "✓ 后端容器已启动"
      else
        echo "→ 启动后端（本机）: mes-backend"
        (
          cd mes-backend
          export DATABASE_URL="postgresql://mes_user:mes_password@localhost:5432/mes_db"
          export REDIS_URL="redis://localhost:6379"
          # 使用 cargo run，若需要 hot reload 可用 cargo-watch（未自动安装）
          cargo run &
        )
        sleep 1
        echo "✓ 后端已在后台启动（cargo run）"
      fi
    else
      echo "✗ 未检测到 cargo，无法在本机启动后端"
      echo "  将使用容器运行后端： docker compose up -d backend"
      docker compose up -d backend
    fi
  else
    echo "✗ 未找到 mes-backend 目录，跳过后端启动"
  fi
else
  echo "✱ 跳过本地后端启动（--no-backend）"
fi

# frontend
if [ "$START_FRONTEND" = true ]; then
  if [ -d "./mes-frontend" ]; then
    if command -v npm &> /dev/null; then
      echo "→ 尝试在本机启动前端（mes-frontend）"
      cd mes-frontend
      npm install
      # 在当前 shell 启动 dev server（确保 MES_FRONTEND_PID 在主进程中可见）
      npm run dev >/tmp/mes_frontend_dev.log 2>&1 &
      MES_FRONTEND_PID=$!
      cd "$ROOT_DIR"
      sleep 5
      # 如果 MES_FRONTEND_PID 为 0 或未设置，或进程不存在，则认为启动失败
      if [ "${MES_FRONTEND_PID:-0}" -eq 0 ] || ! kill -0 "$MES_FRONTEND_PID" >/dev/null 2>&1; then
        echo ""
        echo "⚠ 本地前端 dev server 未能启动（检查 /tmp/mes_frontend_dev.log），将退回使用容器运行前端"
        docker compose up -d frontend
        echo "✓ 已启动前端容器"
      else
        echo "✓ 本地前端 dev server 已在后台启动（PID ${MES_FRONTEND_PID:-unknown}）"
      fi
    else
      echo "✗ 未检测到 npm，无法在本机启动前端"
      echo "  将使用容器运行前端： docker compose up -d frontend"
      docker compose up -d frontend
    fi
  else
    echo "✗ 未找到 mes-frontend 目录，跳过前端启动"
  fi
else
  echo "✱ 跳过本地前端启动（--no-frontend）"
fi

echo ""
echo "开发环境初始化完成。常用命令："
echo "  查看容器日志： docker compose logs -f postgres redis"
echo "  停止容器： docker compose down"
echo "  后端日志（本机）： 在 mes-backend 目录查看 cargo 输出"
echo ""
echo "前端 dev server（本机）通常在 http://localhost:5173"
echo "后端（本机）通常在 http://localhost:8080"
echo ""
exit 0


