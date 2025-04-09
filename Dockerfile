ARG ROS_DISTRO=jazzy
FROM ros:$ROS_DISTRO AS base
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    git \
    libclang-dev \
    python3-pip \
    python3-vcstool \
    && rm -rf /var/lib/apt/lists/*

RUN pip install --break-system-packages colcon-cargo colcon-ros-cargo

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /ros2_ws

RUN mkdir src install

RUN curl -s https://raw.githubusercontent.com/ros2-rust/ros2_rust/main/ros2_rust_jazzy.repos | vcs import src
