## README

### Description
ArrTalk bridges the gap, between the communication of Sonarr/Radarr and Jellyfin.
ArrTalk aimes to be a little helper, to trigger a Jellyfin Library Rescan, if a Sonarr or Radarr Import Job was finished. Without waiting for a Jellyfin Library Rescan to be triggered.

### Running ArrTalk

ArrTalk can be executed either as a standalone binary with the config.toml in the same directory as the binary.
Or as a Docker Container using `docker compose` (recomended).

### Docker Compose

Example `compose.yml`:

```
services:
  arrtalk:
    image: ghcr.io/roccorakete/arrtalk:latest
    environment:
      - SONARR_ENABLE=true
      - SONARR_HOST=<SONARR IP/HOST>
      - SONARR_API=<SONARR API>
      - RADARR_ENABLE=true
      - RADARR_HOST=<RADARR IP/HOST>
      - RADARR_API=<RADARR API>
      - JELLYFIN_ENABLE=true
      - JELLYFIN_HOST=<JELLYFIN IP/HOST>
      - JELLYFIN_API=<JELLYFIN API>
```
### TODO

-[] Support Plex