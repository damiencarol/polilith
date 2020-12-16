# Polilith

Polilith is a tool to detect some common misconfiguration of Docker images

## Rules

### PL001

Root in a container is the same root as on the host machine, but restricted by the docker daemon configuration. No matter the limitations, if an actor breaks out of the container he will still be able to find a way to get full access to the host.

Of course this is not ideal and your threat model can’t ignore the risk posed by running as root.

As such is best to always specify a user:

```
USER hopefullynotroot
```

Note that explicitly setting a user in the Dockerfile is just one layer of defence and won’t solve the whole running as root problem.

Instead one can — and should — adopt a defence in depth approach and mitigate further across the whole stack: strictly configure the docker daemon or use a rootless container solution, restrict the runtime configuration (prohibit --privileged if possible, etc), and so on.