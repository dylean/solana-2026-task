# Solana 开发环境 Docker 镜像

FROM rust:1.92.0

# 设置环境变量
ENV SOLANA_VERSION=1.18.26
ENV ANCHOR_VERSION=0.32.1
ENV NODE_VERSION=20

# 安装系统依赖
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libudev-dev \
    llvm \
    libclang-dev \
    protobuf-compiler \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# 安装 Node.js 和 Yarn
RUN curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION}.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g yarn

# 安装 Solana CLI
RUN sh -c "$(curl -sSfL https://release.solana.com/v${SOLANA_VERSION}/install)" \
    && echo 'export PATH="/root/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc

ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

# 安装 Anchor
RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force \
    && avm install ${ANCHOR_VERSION} \
    && avm use ${ANCHOR_VERSION}

# 配置 Solana 为 devnet
RUN solana config set --url devnet

# 创建工作目录
WORKDIR /workspace

# 暴露常用端口
EXPOSE 8899 8900

# 设置默认命令
CMD ["/bin/bash"]
