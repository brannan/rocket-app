# Deploy to DO Droplet at 137.184.105.64

## Login and copy files
```
scp root@137.184.105.64
```

### Add groups, users, and directories on remote.

```
sudo groupadd rust-svc
sudo useradd -r -s /usr/sbin/nologin -g rust-svc rust-svc

sudo mkdir /opt/rust-svc
sudo chown -R rust-svc:rust-svc /opt/rust-svc
sudo chmod 750 /opt/rust-svc

# Copy files and make sure they are owned by rust-svc:rust:svc
mv rocket-app /opt/rust-svc
sudo chown -R rust-svc:rust-svc /opt/rust-svc

```

### Add rocket-app.system to /etc/systemd/system
```
scp devops/rocket-app.service  root@137.184.105.64:/etc/systemd/system/
ssh root@137.184.105.64
sudo systemctl daemon-reload
sudo systemctl start rocket-app
sudo systemctl enable rocket-app
```

### Stop app and stop it from restarting automatically
```
sudo systemctl stop rocket-app
sudo systemctl disable rocket-app
sudo systemctl daemon-reload
```
