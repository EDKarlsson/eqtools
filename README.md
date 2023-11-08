# EQTools


## Dependencies

### Enigo

#### Debian

```shell
apt-get install libxdo-dev
```

#### Arch

```shell
pacman -S xdotool
```

### Tauri

- [Tauri Deps](https://tauri.app/v1/guides/getting-started/prerequisites)

#### Debian

```shell
sudo apt update && \
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev -y
```

#### Arch

```shell
sudo pacman -Syu && \
sudo pacman -S --needed \
    webkit2gtk \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libvips -y
```

## References

- [OpenMultiBoxing](https://openmultiboxing.org/)
- [ISBoxer](https://isboxer.com/)