# Sandbox and egress baseline (#346)

Fake adapter profile (`SandboxProfile::default`):

- non-root
- no host mounts, Docker socket, SSH agent, home dir
- no inherited environment / secrets
- no network
- writes only to scratch in policy; canonical repo mutation is a separately governed step

## Egress

Default deny. Loopback, RFC1918, link-local, cloud metadata, unique-local IPv6, SOCKS/Tor/onion, and `file:` are `ForbiddenSsrf`. Any other host still requires an explicit destination allowlist on the manifest.

## Later runtimes (not deployed)

Docker/Podman with dropped caps, gVisor or a microVM, Kubernetes NetworkPolicy / Cilium. Documented only. First tests do not start containers.
